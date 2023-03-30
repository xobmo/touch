
This is a demonstration of the haptic grid texture concept. There are three primary components, two visual and one functional.

1. Image display with mouse cursor (on the left)
2. Zoomed-in view of 4 x 4 pixels (on the right)
3. Streaming output to virtual COM port via USB

As the user moves the mouse cursor over the image on the left, the 16 pixels under the cursor are displayed in a zoomed-in
view on the right. Underneath this view is an array of pixel intensity/color values in numeric form (0-255). As those values change, they are streamed out over the virtual COM port as text.

## References for Next Steps

(Thanks to GPT-4)

Proposal for next step is to adapt the existing concept to a 3d system:

Track finger position via OpenPose or MediaPipe
Create a 3d rendering of an object with a texture and/or displacement map using the Bevy (game engine for Rust).
Use RayCasting to find intersection point.
Using the barycentric coordinates of the intersection and interpolation, calculate the uv coordinates.
Calculate the local x/y axis for the texture/displacement map given the orientation of the fingertip.
Sample the texture/displacement map.
Output is the same as current 2d test -- 16 numbers on serial port.

Bevy raycast mod https://docs.rs/bevy_mod_raycast/latest/bevy_mod_raycast/
MediaPipe (hand tracking) Rust crate: https://lib.rs/crates/ux-mediapipe


______


Potential future step: Unity Plugin for custom VR Controller

http://47.94.147.94/unity/Manual/xrsdk-input.html#device-states



