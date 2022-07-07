# YARRT
Yet Another Rust RayTracer

# Diary
Wanted to do something a bit concrete for once, so like a lot of people I tackle the raytracer. I still want to make it standout a bit so I THINK it's gonna be N-dimensional (interpreting a nD space to (n-1)D image (still don't know if it makes sense but it works in my head)).

This readme is starting to feel like a diary but anyway, it's definitly n dimensional. I worked out the maths and it definitely makes sense. We can project a N dimensional image to a N-1 dimensional space. Even the trigonometry with reflection refraction etc will workout. I'm almost positive about it. You can already see how it will look like for dimensions < 3, 3d -> 2d is classic raytracing, 2d -> 1d is a single line, and 1d -> 0d is just a pixel. for 4d->3d and higher, ppm format don't support (obviously). If this project gets big enough, I could try to load a 4d -> 3d ppm image to a 3d space for VR for example. I don't know if it's gonna be easy for the human eye to understand but it could be interesting

I should really have studied more Rust before going in, it looks horrible and unpleasant to work on, I'll try to clean up the code before going any further. As it is now the background is raytraced, an hypersphere is in front of the camera, it gets hit by rays, but the color of the hypersphere is just a random gradient that works from 1->6 dimensions (the normal vector of the ray-hit to the hypersphere multiplied by the primary and secondary colors). It doesn't look great but it runs enough for me.

# How it works
$ cargo run {Output File} {Dimension of the Space} {Resolution}

The image is an hypercube and the {Resolution} is the length of one side, so the number of pixels on the resulting image will be {Resolution}^{Dimension of the Space - 1}

there is also a laughable test-suite, just made it to see how test suites work in Rust, it's pretty cool
