# nalgebra-euler
Extension library for nalgebra with euler and game specific functionality.

## features
- Dedicated Euler Type
- Conversion from/to forward/up Vectors
- NaN Checking

## how to
To use this crate in your project add the following to your `Cargo.toml`:
```
nalgebra-euler = { git = "https://github.com/ko1N/nalgebra-euler" }
```

To access nalgebra-euler and nalgebra simply use:
```
use ::nalgebra_euler::Euler;   // Import Euler from nalgebra-euler
use ::nalgebra_euler::Vector3; // Import Vector3 from nalgebra
```