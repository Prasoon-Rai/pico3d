# Pico 3D

## Introduction

I am building this project to demonstrate how I build a micro 3D graphics library for the raspberry pi pico.

One day while surfing youtube I came accross this amazing video by Tsoding [Demystifies 3D Graphics](https://youtu.be/qjWkNZ0SXfo?si=XWEg81_SU7X2LW7I)
in which he breaks down how graphics apis like Vulkan and Metal actually work under the hood. Of cource they contain
a load of more features but to their core the way they render graphics is the same.

So I thought to myself that it's a great learning opportunity to build one myself, however to make things harder rather
than using HTML and JS (as used by Tsoding in his video), I took the challenge to build it for a raspberry pi pico.

Combining the bare metal nature of the pico and the amazing memory safety guaranteed by rust, I feel like it would make
up for a great learning experience.

## Working

To start from zero, I first wrote a closure function which draws a basic point on the screen.

```
let point = |x: f32, y: f32, style: &_, display: &mut _| {
    let size = 10.0;
    let cords {x, y} = transform(x, y);
    Rectangle::new(Point::new((x - size / 2.0) as i32, (y - size / 2.0) as i32), Size::new(size as u32, size as u32))
        .draw_styled(style, display)
        .unwrap();
};
```
It takes in the _x_ and _y_ co-ordinates to draw the square point at that location.
It also takes in the _style_ as for now I have made 2 styles one matching the background color and one color for the
square itself.

```
let green_box = PrimitiveStyleBuilder::new().fill_color(Rgb666::GREEN).build();
let black_box = PrimitiveStyleBuilder::new().fill_color(Rgb666::BLACK).build();
```

We use these colors interchangebly while also updating the position of the square with each frame to animate the box.

```
loop {
    let initialPos = 0;
    let finalPos = 100;

    for i in initialPos..finalPos {
        point(i - 1, i - 1, &black_box, &mut display);
        point(i, i, &green_box ,&mut display);

        Timer::after_millis(16).await;
    }
    point(finalPos - 1, finalPos - 1, &black_box, &mut display);
}
```

Now moving further we notice that by default the (0, 0) co-ordinates on the screen is the top left corner of the screen,
rather than the middle of the screen. This however, is not how we wanna proceed. So lets change it.
We use the following colsure function along with a struct _cords_.

```
struct cords {
    x: f32,
    y: f32
}

let transform = |x: f32, y: f32| -> cords {
    let rx = (x + 1.0) / 2.0 * (width as f32);
    let ry = (1.0 - (y + 1.0) / 2.0) * (height as f32);

    cords {
        x: rx,
        y: ry
    }
};
```
With the help of this function we can transform the default co-ordinates into our desired co-ordinates.
Further we use these co-ordinates to display our point on the screen using the point function.

Moving further I wrote a function for displaying multiple uniform points on the screen, even making it revolve on the
y-axis by default. (Note that this however can be changed by changing the _rotate_ function we will discuss next.)

```
let mut uniform_points = |points: &[(f32, f32, f32)], speed: f32, size:f32| {
    let dt: f32 = 1.0 / FPS;

    for &(x, y, z) in points {
        let (rx, ry, rz) = rotate(x, y, z, angle);
        point(rx, ry, rz + dz, size, &black_box, &mut display);
    }

    dz += 0.0 * dt;
    angle += speed * core::f32::consts::PI * dt;

    for &(x, y, z) in points {
        let (rx, ry, rz) = rotate(x, y, z, angle);
        point(rx, ry, rz + dz, size, &green_box, &mut display);
    }
    Timer::after_millis((1000.0 / FPS) as u64)
};
```
The above function takes in the array of the points, speed of rotation and the size of each point.