# p3dtools

This is a monorepo of tools for the [Pure3D Engine](https://www.pcgamingwiki.com/wiki/Engine:Pure3D) developed
by Radical Entertainment.

Of note is `crates/p3d2gltf`, which provides the binary p3d2gltf. This binary takes an input .p3d asset file (or folder)
and outputs the models and textures within to a
[gl Transmission Format](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html) file.

Run it in release mode unless you want debug spam.

This is far from finished, barely any pure3d Chunk types are supported, and all of the work has been on
the Simpsons Hit & Run. No idea if the Simpsons Road Rage or Hulk will work.

Also, this is based on both [the Simpsons Hit & Run source code leak](https://archive.org/details/shr_source) and
reverse-engineering work on [Lucas' Pure3D Editor](https://modbakery.donutteam.com/releases/view/lucas-pure3d-editor-4).
