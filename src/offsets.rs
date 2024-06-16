pub mod offsets{
    pub static ACTOR_ID: u64 =  24; //Doesnt change

    pub static  U_WORLD : u64 =  0x81E9240; //0x895D428
    pub static  G_NAMES : u64 =  0x8121468;
    pub static  G_OBJECTS : u64 =  0x8125698;
    pub static  PERSISTENT_LEVEL : u64 =  48;
    pub static ACTOR_ARRAY: u64 =  0xA0; //0x098

    pub static  GAME_INSTANCE : u64 =  448; //UWorld
    pub static  LOCAL_PLAYERS : u64 =  56; //UGameInstance it's a TArray
    pub static PLAYER_CONTROLLER : u64 =  0x30; //UPlayer
    pub static  LOCAL_PAWN: u64 =  0x438; //APlayerController


    //Getting only Player Actors
    pub static  GAME_STATE : u64 =  0x58; //UWorld
    pub static  PLAYER_ARRAY : u64 =  0x3f0; //Engine.GameState
    pub static  PLAYER_STATE : u64 =  0x298;
    pub static  PAWN_PRIVATE : u64 =  0x150; //APlayerState -> maybe Instigator?

    //Drawing
    pub static  CAMERA_MANAGER : u64 =  0x458;
    pub static  CAMERA_CACHE : u64 =  1088; //APlayerCameraManager
    pub static  POV : u64 =  0x10; //FCameraCacheEntry

    //Players Location
    pub static  ROOT_COMPONENT : u64 =  0x168;
    pub static  RELATIVE_LOCATION : u64 =  0x12C;

    pub static FREP_ATTACHMENT : u64 = 0xC8; //AActor First value is the ParentActor
    pub static PROJECTILE_SPEED : u64 = 0x5bc; //ACannon
    pub static FREP_MOVEMENT : u64 = 0x90; //AActor

    pub static UCHAR_INTERACTION_COM : u64 = 0xdf0; //AthenaPlayerCharacter
    pub static CURRENT_INTERACTABLE : u64 = 0x5f0; //UCharacterInteractionComponent
    pub static INTERACTABLE_PARENT_ACTOR : u64 = 0x58; //UInteractableArea

    pub static CHARACTER_MOVEMENT_COMP : u64 = 0x448; //ACharacter

    pub static BASED_MOVEMENT : u64 = 0x458; //ACharacter
    pub static ATTACH_PARENT : u64 = 0xE0; //USceneComponent
    pub static CHILD_ACTOR : u64 = 0x2E8; //UPrimitiveComponent, it's the first value
    pub static PARENT_COMPONENT :u64 = 0x1A0; //AActor

    pub static DAMAGE_LEVEL : u64 = 0x654;
    pub static LEVELS : u64 = 0x150; //UWORLD

    pub static COOKER_COMP : u64 = 0x418;
    pub static COOKING_STATE : u64 = 0x130;
    pub static VISIBLE_COOKED_EXTENT : u64 = 0x54;
    pub static BOX_EXTENT : u64 = 0x05D8; //UBoxComponent default is 75.0 18.0 68.0

    pub static DEFAULT_FOV : u64 = 0x03E0;

    pub static PREDICTION_CLIENT_DATA : u64 = 0x520;
    pub static CLIMBING_COMPONENT : u64 = 0xdf8;
    pub static SERVER_HEIGHT : u64 = 0x194; //InstantLadder

    pub static MAX_SIMULATION_ITER : u64 = 0x2ec;

    pub static MOVEMENT_MODE : u64 = 0x198;

    //IslandService
    pub static ISLAND_DATA_ASSET : u64 = 0x458;
    pub static ISLAND_ARRAY : u64 = 0x480;
    pub static ISLAND_DATA_ENTRIES : u64 = 0x48;
    pub static ISLAND_NAME : u64 = 0x28;
    pub static ISLAND_LOCAL_NAME : u64 = 0xb0;
    pub static ISLAND_BOUNDS_CENTER : u64 = 0x18;
    pub static WORLD_MAP_DATA : u64 = 0x40;
    pub static CAPTURE_PARAMS : u64 = 0x30;
    pub static RENDER_DATA : u64 = 0x848;
    pub static TRANSFORMS : u64 = 0x10;
    pub static MAP_TEXTURE_PATH : u64 = 0x868;
    pub static MARKS : u64 = 0x8f8;
    pub static MARKS_ROTATION : u64 = 0x920;
    pub static MAX_DIST_LADDER : u64 = 0x450;
}
