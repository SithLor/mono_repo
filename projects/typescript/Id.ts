interface id {
    id: `${string}-${string}-${string}-${string}-${string}`
    getId(): `${string}-${string}-${string}-${string}-${string}`
}
class Id implements id {
    id:`${string}-${string}-${string}-${string}-${string}`
    constructor(){
        
        this.id = crypto.randomUUID()
    }
    getId():`${string}-${string}-${string}-${string}-${string}`{
        return this.id
    }
}

export type TbinaryOff = 0;
export type TbinaryOn = 1;
export type TbinaryBooleanTrue = true;
export type TbinaryBooleanFalse = false;
export type TbinaryOffString = "0";
export type TbinaryOnString = "1";
export type TbinaryBooleanTrueString = "true";
export type TbinaryBooleanFalseString = "false";
export type TbinaryBoolean = TbinaryBooleanFalse | TbinaryBooleanTrue;
export type TbinaryNumber = TbinaryOn | TbinaryOff;
export type TbinaryString = TbinaryOnString | TbinaryOffString;
export type TbinaryBooleanString =
  | TbinaryBooleanTrueString
  | TbinaryBooleanFalseString;
export type Tbinary =
  | TbinaryBoolean
  | TbinaryNumber
  | TbinaryString
  | TbinaryBooleanString;
export const BinaryBooleanTrue: TbinaryBooleanTrue = true;
export const BinaryBooleanFalse: TbinaryBooleanFalse = false;
export const BinaryOff: TbinaryOff = 0;
export const BinaryOn: TbinaryOn = 1;
export const BinaryBoolean: TbinaryBoolean = BinaryBooleanFalse;
export const BinaryOnString: TbinaryOnString = "1";
export const BinaryOffString: TbinaryOffString = "0";
export const BinaryBooleanTrueString: TbinaryBooleanTrueString = "true";
export const BinaryBooleanFalseString: TbinaryBooleanFalseString = "false";
export enum EbinaryStateEnum {
  Off = 0,
  On = 1,
  True = 1,
  False = 0,
}
export function BooleanToNumber(state: TbinaryBoolean): TbinaryOn | TbinaryOff {
  if (state === BinaryBooleanTrue) {
    return BinaryOn;
  }
  return BinaryOff;
}
export function BooleanToString(
  state: TbinaryBoolean,
): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
  if (state === BinaryBooleanTrue) {
    return BinaryBooleanTrueString;
  }
  return BinaryBooleanFalseString;
}
export function BooleanToBooleanString(
  state: TbinaryBoolean,
): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
  if (state === BinaryBooleanTrue) {
    return BinaryBooleanTrueString;
  }
  return BinaryBooleanFalseString;
}
export function BooleanToNumberString(
  state: TbinaryBoolean,
): TbinaryOnString | TbinaryOffString {
  if (state === BinaryBooleanTrue) {
    return BinaryOnString;
  }
  return BinaryOffString;
}
export function NumberToBoolean(state: TbinaryOn | TbinaryOff): TbinaryBoolean {
  if (state === BinaryOn) {
    return BinaryBooleanTrue;
  }
  return BinaryBooleanFalse;
}
export function NumberToString(
  state: TbinaryOn | TbinaryOff,
): TbinaryOnString | TbinaryOffString {
  if (state === BinaryOn) {
    return BinaryOnString;
  }
  return BinaryOffString;
}
export function NumberToBooleanString(
  state: TbinaryOn | TbinaryOff,
): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
  if (state === BinaryOn) {
    return BinaryBooleanTrueString;
  }
  return BinaryBooleanFalseString;
}
export function StringToBoolean(
  state: TbinaryBooleanTrueString | TbinaryBooleanFalseString,
): TbinaryBoolean {
  if (state === BinaryBooleanTrueString) {
    return BinaryBooleanTrue;
  }
  return BinaryBooleanFalse;
}
export function StringToNumber(
  state: TbinaryOnString | TbinaryOffString,
): TbinaryOn | TbinaryOff {
  if (state === BinaryOnString) {
    return BinaryOn;
  }
  return BinaryOff;
}
export function StringToBooleanString(
  state: TbinaryBooleanTrueString | TbinaryBooleanFalseString,
): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
  if (state === BinaryBooleanTrueString) {
    return BinaryBooleanTrueString;
  }
  return BinaryBooleanFalseString;
}
export function StringToNumberString(
  state: TbinaryOnString | TbinaryOffString,
): TbinaryOnString | TbinaryOffString {
  if (state === BinaryOnString) {
    return BinaryOnString;
  }
  return BinaryOffString;
}
export interface Ivector<Type> {
    add(...arg:Type[]):void;
    remove(...arg:Type[]):void;
    get():Type[];
    removeAt(index:number):void;
    getAt(index:number):Type;
    clear():void;
    size():number;
}
export class Vector<Type> implements Ivector<Type> {
    data: Type[] ;
    constructor() {
        //if (typeof _data === "undefined") {
        //    this.data = [];
        //} 
        this.data = [];
    }
    add(...arg:Type[]) {
        this.data.push(...arg);
    }
    remove(...arg:Type[]) {
        this.data = this.data.filter((item) => !arg.includes(item));
    }
    get() {
        return this.data;
    }
    removeAt(index: number) {
        this.data.splice(index, 1);
    }
    getAt(index: number) {
        return this.data[index];
    }
    clear() {
        this.data = [];
    }
    size() {
        return this.data.length;
    }
}
// Checks if the given object is an instance of Vector.
export function IsVector<Type>(obj: any): obj is Vector<Type> {
    return obj instanceof Vector;
}
export function CreateVector<Type>() {
    return new Vector<Type>();
}
export namespace binary {
  
  export function BooleanToNumber(
    state: TbinaryBoolean,
  ): TbinaryOn | TbinaryOff {
    if (state === BinaryBooleanTrue) {
      return BinaryOn;
    }
    return BinaryOff;
  }
  export function BooleanToString(
    state: TbinaryBoolean,
  ): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
    if (state === BinaryBooleanTrue) {
      return BinaryBooleanTrueString;
    }
    return BinaryBooleanFalseString;
  }
  export function BooleanToBooleanString(
    state: TbinaryBoolean,
  ): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
    if (state === BinaryBooleanTrue) {
      return BinaryBooleanTrueString;
    }
    return BinaryBooleanFalseString;
  }
  export function BooleanToNumberString(
    state: TbinaryBoolean,
  ): TbinaryOnString | TbinaryOffString {
    if (state === BinaryBooleanTrue) {
      return BinaryOnString;
    }
    return BinaryOffString;
  }

  export function NumberToBoolean(
    state: TbinaryOn | TbinaryOff,
  ): TbinaryBoolean {
    if (state === BinaryOn) {
      return BinaryBooleanTrue;
    }
    return BinaryBooleanFalse;
  }
  export function NumberToString(
    state: TbinaryOn | TbinaryOff,
  ): TbinaryOnString | TbinaryOffString {
    if (state === BinaryOn) {
      return BinaryOnString;
    }
    return BinaryOffString;
  }
  export function NumberToBooleanString(
    state: TbinaryOn | TbinaryOff,
  ): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
    if (state === BinaryOn) {
      return BinaryBooleanTrueString;
    }
    return BinaryBooleanFalseString;
  }

  export function StringToBoolean(
    state: TbinaryBooleanTrueString | TbinaryBooleanFalseString,
  ): TbinaryBoolean {
    if (state === BinaryBooleanTrueString) {
      return BinaryBooleanTrue;
    }
    return BinaryBooleanFalse;
  }
  export function StringToNumber(
    state: TbinaryOnString | TbinaryOffString,
  ): TbinaryOn | TbinaryOff {
    if (state === BinaryOnString) {
      return BinaryOn;
    }
    return BinaryOff;
  }
  export function StringToBooleanString(
    state: TbinaryBooleanTrueString | TbinaryBooleanFalseString,
  ): TbinaryBooleanTrueString | TbinaryBooleanFalseString {
    if (state === BinaryBooleanTrueString) {
      return BinaryBooleanTrueString;
    }
    return BinaryBooleanFalseString;
  }
  export function StringToNumberString(
    state: TbinaryOnString | TbinaryOffString,
  ): TbinaryOnString | TbinaryOffString {
    if (state === BinaryOnString) {
      return BinaryOnString;
    }
    return BinaryOffString;
  }

}











class Vec<T> {
  private data: T[];

  constructor() {
    this.data = [];
  }

  push(item: T): void {
    this.data.push(item);
  }

  pop(): T | undefined {
    return this.data.pop();
  }

}
const c= new Vec<number>();
c.push(1);
c.pop()
import * as fs from 'fs';


function binToJS(fname,bin_path,target_path){
    const _Buffer = fs.readFileSync(bin_path)
    const _BufferObj = Buffer.from(_Buffer);
    const _IntArray = JSON.parse(JSON.stringify(_BufferObj)).data;
    fs.writeFileSync(target_path+"hex.js", `export const WMIC_DATA = "${wmicBufferObj.toString('hex')}`);
    fs.writeFileSync(target_path+"bin.js", `export const WMIC_DATA = [${wmicIntArray};]`);

}
const wmicBuffer = fs.readFileSync(path)
const wmicBufferObj = Buffer.from(wmicBuffer);
const wmicIntArray = JSON.parse(JSON.stringify(wmicBufferObj)).data;
fs.writeFileSync('./src/app/WMIC.HEX.js', `export const WMIC_DATA = "${wmicBufferObj.toString('hex')}`);
fs.writeFileSync('./src/app/WMIC.INT.js', `export const WMIC_DATA = [${wmicIntArray};]`);



