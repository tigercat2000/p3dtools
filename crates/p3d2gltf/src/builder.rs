use std::collections::HashMap;

use eyre::eyre;
use gltf_json::{buffer, validation::Validate, Buffer, Index, Root};
use p3dparse::chunk::data::kinds::shared::{Colour, Vector2, Vector3};

/// This trait is used to add something to a list and returns the index it was put at.
trait PushReturnIndex {
    type Output;
    /// This adds something to a list and returns the index it was put at.
    fn push_indexed(&mut self, data: Self::Output) -> usize;
}

impl<T> PushReturnIndex for Vec<T> {
    type Output = T;

    fn push_indexed(&mut self, data: Self::Output) -> usize {
        self.push(data);
        self.len() - 1
    }
}

trait WriteGltf {
    fn write_data(&self, out: &mut Vec<u8>);
}

impl WriteGltf for [u8] {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.extend(self);
    }
}

impl WriteGltf for u8 {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.push(*self);
    }
}

impl WriteGltf for [u16] {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.extend(self.iter().flat_map(|f| f.to_le_bytes()))
    }
}

impl WriteGltf for u16 {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.extend(self.to_le_bytes())
    }
}

impl WriteGltf for [u32] {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.extend(self.iter().flat_map(|f| f.to_le_bytes()))
    }
}

impl WriteGltf for u32 {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.extend(self.to_le_bytes())
    }
}

impl WriteGltf for [f32] {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.extend(self.iter().flat_map(|f| f.to_le_bytes()))
    }
}

impl WriteGltf for f32 {
    fn write_data(&self, out: &mut Vec<u8>) {
        out.extend(self.to_le_bytes())
    }
}

#[derive(Debug, Default)]
#[allow(non_camel_case_types)]
pub struct glTFBuilder {
    root: Root,
    unencoded_buffers: HashMap<Index<Buffer>, Vec<u8>>,
}

/// Internal functions.
impl glTFBuilder {
    /// This will throw an [`Result::Err`] if the internal [`gltf_json`] validation of our [`Root`] fails.
    /// Used to ensure the state is always valid after every public function call.
    ///
    /// During the process of the call and during internal calls, the state is allowed to be invalid, as we hold a &mut self.
    /// This is unfortunately necessary because some operations require insertions into two different parts of the struct,
    /// which cannot be done in parallel.
    fn check_validity(&self) -> crate::Result<()> {
        let mut errors = Vec::new();
        self.root
            .validate(&self.root, gltf_json::Path::new, &mut |path, error| {
                errors.push((path(), error))
            });

        // Make sure our mesh is valid!
        if !errors.is_empty() {
            return Err(eyre!("{:#?}", errors));
        }

        Ok(())
    }

    /// This helper returns an immutable reference to
    /// both the real unencoded data from [`glTFBuilder::unencoded_buffers`]
    /// as well as the [`Buffer`] entry, so that internal functions can view the
    /// data as if it were combined.
    fn get_buffer(&self, buffer: Index<Buffer>) -> (&Vec<u8>, &Buffer) {
        (
            self.unencoded_buffers.get(&buffer).unwrap(),
            self.root.buffers.get(buffer.value()).unwrap(),
        )
    }

    /// This helper returns a mutable reference to
    /// both the real unencoded data from [`glTFBuilder::unencoded_buffers`]
    /// as well as the [`Buffer`] entry, so that internal functions can mutate the
    /// data as if it were combined.
    fn get_buffer_mut(&mut self, buffer: Index<Buffer>) -> (&mut Vec<u8>, &mut Buffer) {
        (
            self.unencoded_buffers.get_mut(&buffer).unwrap(),
            self.root.buffers.get_mut(buffer.value()).unwrap(),
        )
    }

    /// This creates a new [`Buffer`] backed by a [`Vec<u8>`]
    /// in [`glTFBuilder::unencoded_buffers`].
    fn create_buffer(&mut self, name: Option<&str>) -> Index<Buffer> {
        let buffer_type = Buffer {
            byte_length: 0,
            name: name.map(|x| x.to_owned()),
            uri: None,
            extensions: None,
            extras: Default::default(),
        };

        let index = Index::new(self.root.buffers.push_indexed(buffer_type) as u32);

        self.unencoded_buffers.insert(index, Vec::new());

        index
    }

    /// This inserts additional data into an existing [`Buffer`] and
    /// creates a new [`buffer::View`] to that data, then returns
    /// an index to the newly created [`buffer::View`].
    fn insert_raw_data(
        &mut self,
        name: Option<&str>,
        buffer: Index<Buffer>,
        data: &[u8],
    ) -> Index<buffer::View> {
        let (real_buffer, buffer_type) = self.get_buffer_mut(buffer);

        // The index offset of the buffer view into this buffer will be whatever our current
        // length is. However, the glTF spec asks that we simply don't encode the offset if
        // it is 0, so we have to account for that.
        let potential_offset = real_buffer.len() as u32;
        let byte_offset = if potential_offset != 0 {
            Some(potential_offset)
        } else {
            None
        };

        // Write the data to the end of the real unencoded vec.
        real_buffer.extend(data);

        // Always synchronize the buffer_type length.
        buffer_type.byte_length = real_buffer.len() as u32;

        let buffer_view = buffer::View {
            buffer,
            byte_length: data.len() as u32,
            byte_offset,
            byte_stride: None,
            name: name.map(|x| x.to_owned()),
            target: None,
            extensions: None,
            extras: Default::default(),
        };

        Index::new(self.root.buffer_views.push_indexed(buffer_view) as u32)
    }

    /// Most accessor data types must be aligned. This has to be accounted for
    /// by adding padding to our [`Buffer`]s. This will automatically pad if needed.
    ///
    /// This won't cause any issues, as [`Buffer`]s are always accessed through
    /// [`BufferView`]s which we ensure are valid when they are created,
    /// and this function will never modify any data already referenced.
    ///
    /// Example: an f32 accessor will ask:
    /// ```no_compile
    /// self.pad_to_alignment(index, std::mem::size_of::<f32>());
    /// ```
    fn pad_to_alignment(&mut self, buffer: Index<Buffer>, alignment: usize) {
        let (real_buffer, buffer_type) = self.get_buffer_mut(buffer);

        // This may do nothing if it is already padded, that's okay.
        while real_buffer.len() % alignment != 0 {
            // Zero-pad.
            real_buffer.push(0);
        }

        // Update the byte length.
        buffer_type.byte_length = real_buffer.len() as u32;
    }

    /// Steps to build:
    /// - Base64 encode [`glTFBuilder::unencoded_buffers`] and place them in their respective [`Buffer::uri`] fields.
    /// - Return [`serde_json::to_string(root)`].
    fn build_internal(self) -> String {
        todo!()
    }
}

/// Public functions.
impl glTFBuilder {
    /// Constructs a new gltfBuilder with a valid, default internal representation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Finalize the [`glTFBuilder`] and return the json encoded [`Root`].
    pub fn build(self) -> String {
        self.build_internal()
    }
}

#[cfg(test)]
mod tests {
    use gltf_json::{Buffer, Index};

    use crate::builder::glTFBuilder;

    fn helper_setup_buffer(builder: &mut glTFBuilder) -> Index<Buffer> {
        let idx = builder.create_buffer(Some("Test Buffer"));
        assert_eq!(idx.value(), 0);
        idx
    }

    #[test]
    fn test_default_is_valid() {
        glTFBuilder::new().check_validity().unwrap()
    }

    #[test]
    fn test_buffer_is_created() {
        let mut builder = glTFBuilder::new();
        let index = helper_setup_buffer(&mut builder);

        assert_eq!(builder.root.buffers.len(), 1);

        let (_, new_buffer) = &builder.get_buffer(index);

        assert_eq!(new_buffer.byte_length, 0);
        assert_eq!(new_buffer.name, Some("Test Buffer".into()));
        assert_eq!(new_buffer.uri, None);

        builder.check_validity().unwrap()
    }

    #[test]
    fn test_buffer_view_is_created() {
        let mut builder = glTFBuilder::new();
        let buffer_idx = helper_setup_buffer(&mut builder);

        let buffer_view_idx = builder.insert_raw_data(Some("Test Data"), buffer_idx, &[1, 2, 3]);

        assert_eq!(buffer_view_idx.value(), 0);
        assert_eq!(builder.root.buffer_views.len(), 1);

        let new_buffer_view = &builder.root.buffer_views[0];

        assert_eq!(new_buffer_view.buffer.value(), 0);
        assert_eq!(new_buffer_view.byte_length, 3);
        assert_eq!(new_buffer_view.byte_offset, None);
        assert_eq!(new_buffer_view.name, Some("Test Data".into()));

        let existing_buffer = &builder.root.buffers[0];
        assert_eq!(existing_buffer.byte_length, 3);

        builder.check_validity().unwrap()
    }

    #[test]
    fn test_multiple_views() {
        let mut builder = glTFBuilder::new();
        let buffer_idx = helper_setup_buffer(&mut builder);
        let buffer_view_idx_1 = builder.insert_raw_data(Some("Test Data"), buffer_idx, &[1, 2, 3]);
        assert_eq!(buffer_view_idx_1.value(), 0);
        let buffer_view_idx_2 =
            builder.insert_raw_data(Some("Test Data 2"), buffer_idx, &[4, 5, 6]);
        assert_eq!(buffer_view_idx_2.value(), 1);

        let buffer_views = &builder.root.buffer_views;
        assert_eq!(buffer_views.len(), 2);
        assert_eq!(buffer_views[0].byte_length, 3);
        assert_eq!(buffer_views[1].byte_length, 3);
        assert_eq!(buffer_views[1].byte_offset, Some(3));

        assert_eq!(builder.root.buffers.len(), 1);
        let (real_buffer, buffer) = builder.get_buffer(buffer_idx);

        assert_eq!(buffer.byte_length, 6);
        assert_eq!(buffer.uri, None);
        assert_eq!(real_buffer.len() as u32, buffer.byte_length);
        assert_eq!(real_buffer.first(), Some(&1));
        assert_eq!(real_buffer.last(), Some(&6));

        builder.check_validity().unwrap()
    }

    #[test]
    fn test_padding() {
        let mut builder = glTFBuilder::new();
        let buffer_idx = helper_setup_buffer(&mut builder);
        let buffer_view = builder.insert_raw_data(None, buffer_idx, &[1, 2]);

        let (real_buffer, _) = builder.get_buffer(buffer_idx);
        assert_eq!(real_buffer.len(), 2);

        builder.pad_to_alignment(buffer_idx, std::mem::size_of::<f32>());

        let (real_buffer, _) = builder.get_buffer(buffer_idx);
        assert_eq!(real_buffer.len(), 4);

        assert_eq!(
            builder
                .root
                .buffer_views
                .get(buffer_view.value())
                .unwrap()
                .byte_length,
            2
        );

        // Should still be valid at this point.
        builder.check_validity().unwrap();

        let new_buffer_view = builder.insert_raw_data(None, buffer_idx, &[3, 4]);
        let (real_buffer, _) = builder.get_buffer(buffer_idx);
        assert_eq!(real_buffer.len(), 6);

        assert_eq!(
            builder
                .root
                .buffer_views
                .get(new_buffer_view.value())
                .unwrap()
                .byte_offset,
            Some(4)
        );

        // Should still be valid at this point.
        builder.check_validity().unwrap();
    }
}
