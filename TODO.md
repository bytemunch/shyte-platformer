# TODO

## BUGS

BUG: player sticks to ceilings

BUG: enemies don't bounce off eachother

## TODOS

TODO: multiple ending cutscenes based on respect meter

## NOTES

NOTE: Parent/Child hierarchies are logical. It just adds the appropriate Parent/Children components to each entity. 
    EG child gets the Parent component pointing to parent entity.

NOTE: Bundles are only used when instantiating. Components are all that is attached to entities.

NOTE: Adding multiple of the same component overwrites previous component.

## LATER

TODO: animate chalk

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

REFAC: font as resource

REFAC: cutscene / dialogue system w/ world-screen transform for speech

REFAC: bundled entity child bundles, for childbuilder.spawn(ChildBundle::default())
    like enemy/player child spritebundle bundle

REFAC: kinematic physics ordering/labelling

REFAC: button clicking functions can be one function with some component matching?

REFAC: level vec for platforms and enemies, iterate and spawn instead of writing out helper fns lmao
