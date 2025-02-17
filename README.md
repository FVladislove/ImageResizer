So, I created this project to potentially speed up my colleague's image resizing process (and to learn rust).
But I was curious when Rust code was slower than Python.
At first, I thought it was because of my poor language knowledge or some algorithm differences,
but from what I found and realized, it was all because Pillow library actually uses C under the hood,
when the image crate uses rust.
But I might be wrong)