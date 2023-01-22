# TODO

## BUGS

BUG: cannot jump when pusing against wall. Character controller offset?

BUG: player sticks to walls AND ceilings

BUG: enemies don't bounce off eachother

## TODOS

TODO: intro, multiple ending cutscenes based on respect meter

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

## REFACS

### A.K.A "NEVERTODOS"

`it don't have to be perfect just ship SOMETHING for ONCE jeeeeeeeeeeez`

REFAC: bundled entity child bundles, for childbuilder.spawn(ChildBundle::default())
    like enemy/player child spritebundle bundle

REFAC: kinematic physics ordering/labelling

REFAC: button clicking functions can be one function with some component matching?

REFAC: level vec for platforms and enemies, iterate and spawn instead of writing out helper fns lmao
