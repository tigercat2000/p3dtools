use std::collections::HashMap;

use eyre::eyre;
use gltf_json::{buffer, validation::Validate, Buffer, Index, Root};

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

#[derive(Debug, Default)]
#[allow(non_camel_case_types)]
pub struct glTFBuilder {
    root: Root,
    unencoded_buffers: HashMap<Index<Buffer>, Vec<u8>>,
}

impl glTFBuilder {
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

    fn get_buffer(&mut self, buffer: Index<Buffer>) -> (&Vec<u8>, &Buffer) {
        (
            self.unencoded_buffers.get(&buffer).unwrap(),
            self.root.buffers.get(buffer.value()).unwrap(),
        )
    }

    fn get_buffer_mut(&mut self, buffer: Index<Buffer>) -> (&mut Vec<u8>, &mut Buffer) {
        (
            self.unencoded_buffers.get_mut(&buffer).unwrap(),
            self.root.buffers.get_mut(buffer.value()).unwrap(),
        )
    }

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

    fn insert_data(
        &mut self,
        name: Option<&str>,
        buffer: Index<Buffer>,
        data: &[u8],
    ) -> Index<buffer::View> {
        let (real_buffer, buffer_type) = self.get_buffer_mut(buffer);

        let potential_offset = real_buffer.len() as u32;
        let byte_offset = if potential_offset != 0 {
            Some(potential_offset)
        } else {
            None
        };
        real_buffer.extend(data);
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
}

impl glTFBuilder {
    pub fn new() -> Self {
        Self::default()
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

        let new_buffer = &builder.root.buffers[0];

        assert_eq!(new_buffer.byte_length, 0);
        assert_eq!(new_buffer.name, Some("Test Buffer".into()));
        assert_eq!(new_buffer.uri, None);

        builder.check_validity().unwrap()
    }

    #[test]
    fn test_buffer_view_is_created() {
        let mut builder = glTFBuilder::new();
        let buffer_idx = helper_setup_buffer(&mut builder);

        let buffer_view_idx = builder.insert_data(Some("Test Data"), buffer_idx, &[1, 2, 3]);

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
        let buffer_view_idx_1 = builder.insert_data(Some("Test Data"), buffer_idx, &[1, 2, 3]);
        assert_eq!(buffer_view_idx_1.value(), 0);
        let buffer_view_idx_2 = builder.insert_data(Some("Test Data 2"), buffer_idx, &[4, 5, 6]);
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
    }
}
