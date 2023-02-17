# TODO

## BUGS

BUG: player sticks to ceilings

BUG: font spills over button graphic for resume

BUG: win screen ok button text not centered

BUG: cutscene text appears at wrong place at different window sizes

BUG: win screen text appears at wrong place at different window sizes. use flexbox better

BUG: returning to menu from end screen breaks background scaling

## TODOS

TODO: use Text2dBundle for dialogue

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

REFAC: font as resource / default font

REFAC: cutscene / dialogue system

REFAC: kinematic physics ordering/labelling

REFAC: level vec for platforms and enemies, iterate and spawn instead of writing out helper fns lmao

REFAC: background

REFAC: respect meter should resize with window