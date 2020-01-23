## emitbrown

This project is a fork of [RustyEmitter](https://github.com/kentaromiura/RustyEmitter) and it uses [hashbrown](https://github.com/Amanieu/hashbrown) to replace `std::collections::HashMap`.
It has also some small api changes

## usage

```rust
extern crate emitbrown;
extern crate hashbrown;

use emitbrown::{Events, Emitter};
use hashbrown::HashMap;

fn main(){
    let (mut emitter, callback) = (
        // create a new emitter instance
        Emitter::new(),

        // creating the handler in the same lifetime
        Box::new(|data:& mut HashMap<String, String>| {
            println!("IT WORKS!");
            for (key, value) in data {
                println!("{}: {}", key, value);
            }
        }
    ));

    // listen to the "IT WORKS" event
    emitter.on("IT WORKS".to_string(), callback);

    // fire the "IT WORKS" event with an empty HashMap;
    emitter.emit("IT WORKS".to_string(), &mut HashMap::new());

    // fire it again passing some more data
    let mut data : HashMap<String, String> = HashMap::new();

    data.insert("some data".to_string(), "here".to_string());
    emitter.emit("IT WORKS".to_string(), &mut data);
}
```
