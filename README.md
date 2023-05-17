# Virtual Camera

Goal of this project is implementing a simple rendering engine, which allows moving around a virtual world.

The project consists of 3 iterations:

1. Implement fundamental operations to allow observer to fully explore the world
2. Implement elimination of covered elements (or in gamer words - drawing objects without wall hack)
3. Implement lighting

## Controls

### Movement

| Direction | Key        |
|-----------|------------|
| Forward   | W          |
| Backward  | S          |
| Left      | A          |
| Right     | D          |
| Up        | Arrow up   |
| Down      | Arrow down |

### Zoom

| Zoom  | Key   |
|-------|-------|
| In    | Q     |
| Out   | E     |
| Reset | Space |

### Looking around

| Look direction | Key |
|----------------|-----|
| Up             | I   |
| Down           | K   |
| Left           | J   |
| Right          | L   |

### Tilting camera

| Tilt direction | Key |
|----------------|-----|
| Left           | U   |
| Right          | O   |

## How does it work
Camera is controlled by 4 parameters:
- `position` - current coordinates of the observer
- `forward`, `up` - vectors used to orient the camera
- `fov` - current field of view

The main steering loop adjusts the parameters accordingly:
- movement - up/down and forward/back movement is easy since we keep track of vectors for these directions, for left/right we can take cross product of `forward` with `up` to obtain the needed vector
- looking around - rotate the `forward` vector around the axis of either `up` vector (looking left/right) or `forward * up` vector (looking up/down)
- tilting camera - rotate the `up` vector around the axis of `forward` vector
- zoom - decrement or increment `fov` accordingly (with some constraints)

After parameters are adjusted, `cgmath` does all the heavy lifting with calculating transformation matrices:
```rust
// AR, NEAR, FAR - aspect ratio and clipping planes
let view = Matrix4::look_to_lh(position, forward, up);
let projection = cgmath::perspective(Deg(fov), AR, NEAR, FAR);
```
