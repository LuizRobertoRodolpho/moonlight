# Roadmap - UDP (14/04/2023)
- KeyDown move/render
- Handle player objects on server side
- Implement/improve Structs and Traits for:
    - Client: 
        - 'ClientNetwork'
        - 'Rendering'
        - 'PlayerRules'
    - Server:
        - 'ServerNewtork'
        - 'GameCore'
    - 'BaseNetwork'
- Start Screen
    - Number of users playing
    - Enter nickname
    - Port (optional)
- New game window
    - Embedded background image
- Logger + LogLevel
- WebAssembly UI

## Refactoring & Known bugs
- Move some objects to the stack
- Adjust some objects in the heap and create pointers to stack
- Decide about server tokio::spawn or original implementation
- Review protocol message standards
- UI render and control based on'Widgets'
- Log cleanup
- [Improve error handling](https://doc.rust-lang.org/error_codes/error-index.html)

## References
- [FLTK](https://github.com/fltk-rs)
    - [Samples](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples)
    - [3rd-part Samples](https://github.com/wyhinton/FLTK-RS-Examples)
- [Online Exercises](https://exercism.org/tracks/rust)
