# TODO

## BUGS

BUG: DONTCARE: returning to menu from end screen breaks background scaling temporarily

BUG: DONTCARE: resizing while cutscene is active breaks background

## TODOS

TODO: Seperate SFX & BGM mute buttons

TODO: Debounce mute button

## SOUNDS

laughing
dialogue

## NOTES

NOTE: Parent/Child hierarchies are logical. It just adds the appropriate Parent/Children components to each entity. 
    EG child gets the Parent component pointing to parent entity.

NOTE: Bundles are only used when instantiating. Components are all that is attached to entities.

NOTE: Adding multiple of the same component overwrites previous component.

## LATER

TODO: animate chalk

TODO: win screen OK button fade in

TODO: tween player to end cutscene position

TODO: cinematic bars

## SCOPE CREEP

CREEP: level editor

    CREEP: Editor tools

    CREEP: editor show selected

    CREEP: Editor save / load

    CREEP: Editor menu, opened with escape, save, load, quit, menu

    CREEP: Box regenerate meshes

CREEP: level loader

CREEP: 9-Patch box drawing

-- look into sensors?
--- experiment with sensors as child objects in scratchpad, as they seem to behave oddly

## REFACS

### A.K.A "NEVERTODOS"

`it don't have to be perfect just ship SOMETHING for ONCE jeeeeeeeeeeez`

REFAC: cutscene / dialogue system

REFAC: kinematic physics ordering/labelling

REFAC: level vec for platforms and enemies, iterate and spawn instead of writing out helper fns lmao

REFAC: background

REFAC: respect meter should resize with window

REFAC: game title not centered

REFAC: audio systems seperated from where they're jammed in currently. Events based audio?