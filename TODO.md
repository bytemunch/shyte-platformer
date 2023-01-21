# TODO

## REFACS

REFAC: bundled entity child bundles, for childbuilder.spawn(ChildBundle::default())
    like enemy/player child spritebundle bundle

REFAC: kinematic physics ordering/labelling

REFAC: button clicking functions can be one function with some component matching?

## BUGS

BUG: cannot jump when pusing against wall. Character controller offset?

BUG: player sticks to walls

BUG: jittery enemy movement SOMETIMES, ordering issue. (likely all movement is 
jittery, camera moving masks it)

BUG: enemy still moves when paused

## TODOS

TODO: Coyote time: ignore gravity while active

TODO: fill in box graphics

TODO: animate chalk

TODO: respect meter, fills as percentage of enemies killed

TODO: full respect meter win state

## SCOPE CREEP

CREEP: level editor

    CREEP: Editor tools

    CREEP: editor show selected

    CREEP: Editor save / load

    CREEP: Editor menu, opened with escape, save, load, quit, menu

    CREEP: Box regenerate meshes

CREEP: level loader

CREEP: 9-Patch box drawing
