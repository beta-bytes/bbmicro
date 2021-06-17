# BBMicro

A minimalist game framework in rust, inspired by PICO8.

## How it use it

Currently the best way is to fork this repo or make a branch and implement ```Game1```. (We will improve this :) ).

The game frame rate is locked at 30fps. You can depend on this.

### Sound

Sound is coming soon!

### Graphics

The resolution of the game is 128 x 128.

There is one spritesheet: ```sprites.png```, you should edit / replace this with graphics you'd like to use in your game. 
There is a ```camera``` method that you can use as a virtual camera. All the drawing commands can respect it (some are optional).
```spr``` draws sprites from the spritesheet onto the screen. The convention is sprite 0 is the top left corner, it then 
works its way accross and then down in 8x8 blocks. There are 16 tiles in each row and column.
```print``` prints strings onto the screen. Currently only uppercase is supported. (We will fix this soon).


#### Mapping

Currently mapping is very very simple. We have a default 256 x 256 map. To draw it use ```map```. It is initialized the 0 sprite.

To modify the map, use ```mset```. To check the map use ```mget```. In the future we may want to add simple feature for loading
maps, or layering, but we also don't want to be too prescriptive. For now the game can do its own "loading" via ```mset```.

### Input

Input is a, b, up, down, left, right. We can work on providing a mechanism to map these to controllers or different keys.
You acces input via the ```btn``` and ```btnp``` methods.
