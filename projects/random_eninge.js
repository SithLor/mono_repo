function random_choice(arr){
    return arr[Math.floor(Math.random() * arr.length)]
}
function random_int(min,max){
    return Math.floor(Math.random() * (max - min + 1)) + min
}
function random_float(min,max){
    return Math.random() * (max - min) + min
}
function random_bool(){
    return Math.random() >= 0.5
}


const quality_min = -15
const quality_max = 15

//https://wiki.automationgame.com/index.php?title=Engine_Design#Block_Material
//block and head
function gen_eninge(){
    /// base
    const type = ["V","Inline","Boxer",]
    const block_material = ["Cast Iron","Aluminium","Aluminium Silicate (AlSi)","Magnesium"]
    const HEAD_VALUES = ["Pushrod [OHV]","Direct-Acting Overhead Cam [DaOHC]","Single Overhead Cam [SOHC]","Dual Overhead Cam [DOHC]"]
    const head_material = ["Cast Iron","Aluminium","Aluminium Silicate (AlSi)"]
    const FINAL_VALUES = random_choice(HEAD_VALUES);
    const FINAL_HEAD_MATERIAL = random_choice(head_material);
    const FINAL_BLOCK_MATERIAL = random_choice(block_material);
    const FINAL_TYPE = random_choice(type)
    let FINAL_CYLINDERS = 0;
    let FINAL_V_ANGLE = 0;
    let FINAL_DISPLACEMENT = Math.floor(Math.random() * 10) + 1; // random number between 1 and 10
    let FINAL_BORE = FINAL_DISPLACEMENT; // bore matches displacement
    let FINAL_STROKE;
    
    // calculate stroke based on displacement, bore, and cylinder count
    if (FINAL_TYPE == "Boxer"){
        FINAL_CYLINDERS = random_choice([4,6]);
        FINAL_STROKE = FINAL_DISPLACEMENT / (Math.PI * Math.pow(FINAL_BORE / 2, 2) * FINAL_CYLINDERS);
    }
    if (FINAL_TYPE == "Inline"){
        FINAL_CYLINDERS = random_choice([3,4,5,6]);
        FINAL_STROKE = FINAL_DISPLACEMENT / (Math.PI * Math.pow(FINAL_BORE / 2, 2) * FINAL_CYLINDERS);
    }
    if (FINAL_TYPE == "V"){
        const angle = random_choice([60,90]);
        FINAL_V_ANGLE = angle;
        if (angle == 60){
            FINAL_CYLINDERS = random_choice([6,8,12]);
        }
        if (angle == 90){
            FINAL_CYLINDERS= random_choice([6,8,10,16]);
        }
        FINAL_STROKE = FINAL_DISPLACEMENT / (Math.PI * Math.pow(FINAL_BORE / 2, 2) * FINAL_CYLINDERS);
    }
    /// base

    /// bottem end
    const crank_material = ["Cast Iron","Forged Steel","Billet Steel"]
    const crank_type = ["Flat Plane","Cross Plane"]
    const FINAL_CRANK_MATERIAL = random_choice(crank_material);
    let FINAL_CRANK_TYPE;
    if (FINAL_TYPE == "V"){
        if (FINAL_V_ANGLE == 60){
            FINAL_CRANK_TYPE = "Cross Plane"
        }
        if (FINAL_V_ANGLE == 90){
            FINAL_CRANK_TYPE = random_choice(crank_type)
        }
    }
    if (FINAL_TYPE == "Inline"){
        FINAL_CRANK_TYPE = "NO CRANK TYPE"
    }
    if (FINAL_TYPE == "Boxer"){
        FINAL_CRANK_TYPE = "NO CRANK TYPE"
    }
    const conrods_material = ["Cast","Heavy Duty Cast","Heavy Duty Forged","Lightweight Forged","Lightweight Titanium"]
    const FINAL_CONRODS_MATERIAL = random_choice(conrods_material);

    const pistons_material = ["Cast","Heavy Duty Cast","Forged","Hypereutectic Cast","Low Friction Cast","Lightweight Forged"]
    const FINAL_PISTONS_MATERIAL = random_choice(pistons_material);
    /// bottem end

    /// top end
    const vvl_option = ["all cams","intake only","none"]
    const FINAL_COMPRESSION_RATIO = random_float(5.0,16.0).toFixed(1);
    const FINAL_CAM_PROFILE = random_int(0,100);
    const FINAL_VVL_OPTIONS = random_choice(vvl_option);
    /// top end

    /// fuel
    const fuel_option = ["Carburetors","Injection"]
    const FINAL_FUEL = random_choice(fuel_option);
    if (FINAL_FUEL == "Carburetors"){
        const carburetor_option = ["Single Barrel","Single Barrel Eco Carburetor","Quad Barrel Carburetor","DCOE Carburetor"]
        const FINAL_CARBS = random_choice(carburetor_option);
    }
    if (FINAL_FUEL == "Injection"){
        const injection_option = ["Single Point Injection","Multi Point Injection","Direct Injection"]
        const FINAL_INJECTION = random_choice(injection_option);
    }
    /// fuel
    let engine = {
        base:{
            "type": FINAL_TYPE,
            "cylinders": FINAL_CYLINDERS,
            "v_angle": FINAL_V_ANGLE,
            "displacement": FINAL_DISPLACEMENT,
            "bore": FINAL_BORE,
            "stroke": FINAL_STROKE,
            "head_values": FINAL_VALUES,
        
            "head_material": FINAL_HEAD_MATERIAL,
            "block_material": FINAL_BLOCK_MATERIAL,
            "quality": random_int(quality_min,quality_max),
        },
        bottem_end: {
            "crank_material": FINAL_CRANK_MATERIAL,
            "crank_type": FINAL_CRANK_TYPE,
            "conrods_material": FINAL_CONRODS_MATERIAL,
            "pistons_material": FINAL_PISTONS_MATERIAL,
            "quality": random_int(quality_min,quality_max),
        },
        top_end: {
            "compression_ratio": FINAL_COMPRESSION_RATIO,
            "cam_profile": FINAL_CAM_PROFILE,
            "vvl_options": FINAL_VVL_OPTIONS,
            "quality": random_int(quality_min,quality_max),
        },
        fuel:{

        }
    }
    return engine
}


console.log(gen_eninge())
