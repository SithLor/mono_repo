namespace java {
    export interface Program {
        main();
    }
    export abstract class Program {
        constructor(){
            let RETURN_CODE:STATUS = this.main();
            switch(RETURN_CODE){
                case STATUS.SUCCESS:
                    console.log('Success');
                    break;
                case STATUS.FAILED:
                    console.log('Failed');
                    break;
            }
        }
        //@ts-ignore
        public abstract main();
    }
    export class ArrayList<T> {
        private array: T[] = [];
    
        public add(item: T): void {
            this.array.push(item);
        }
    
        public get(index: number): T | undefined {
            return this.array[index];
        }
    
        public remove(index: number): void {
            this.array.splice(index, 1);
        }
    
        public isEmpty(): boolean {
            return this.array.length === 0;
        }
    
        public size(): number {
            return this.array.length;
        }
    }
    
    export enum STATUS {
        SUCCESS = 0,
        FAILED = 1
    }
    export class List<E>{
        private array: E[] = [];
    
        public add(item: E): void {
            this.array.push(item);
        }
        public addAll(items: E[]): void {
            this.array = this.array.concat(items);
        }
        public addAllAt(items: E[], index: number): void {
            this.array.splice(index, 0, ...items);
        }
        public addAt(item: E, index: number): void {
            this.array.splice(index, 0, item);
        }

        public clear(): void {
            this.array = [];
        }
        public contains(item: E): boolean {
            return this.array.includes(item);
        }
        public containsAll(items: E[]): boolean {
            return items.every(item => this.array.includes(item));
        }
        public get(index: number): E | undefined {
            return this.array[index];
        }
        public indexOf(item: E): number {
            return this.array.indexOf(item);
        }
        public removeAll(items: E[]): void {
            this.array = this.array.filter(item => !items.includes(item));
        }
        public retainAll(items: E[]): void {
            this.array = this.array.filter(item => items.includes(item));
        }
        public set(index: number, item: E): void {
            this.array[index] = item;
        }
        public sort(compareFn?: (a: E, b: E) => number): void {
            this.array.sort(compareFn);
        }
        public subList(fromIndex: number, toIndex: number): E[] {
            return this.array.slice(fromIndex, toIndex);
        }
        public toArray(): E[] {
            return this.array;
        }

        public remove(index: number): void {
            this.array.splice(index, 1);
        }
    
        public isEmpty(): boolean {
            return this.array.length === 0;
        }
    
        public size(): number {
            return this.array.length;
        }
    }
    export class Map<K, V> {
        private keys: K[] = [];
        private values: V[] = [];
    
        public put(key: K, value: V): void {
            const index = this.keys.indexOf(key);
            if (index === -1) {
                this.keys.push(key);
                this.values.push(value);
            } else {
                this.values[index] = value;
            }
        }
    
        public get(key: K): V | undefined {
            const index = this.keys.indexOf(key);
            if (index === -1) {
                return undefined;
            }
            return this.values[index];
        }
    
        public remove(key: K): void {
            const index = this.keys.indexOf(key);
            if (index === -1) {
                return;
            }
            this.keys.splice(index, 1);
            this.values.splice(index, 1);
        }
    
        public isEmpty(): boolean {
            return this.keys.length === 0;
        }
    
        public size(): number {
            return this.keys.length;
        }
    }
}
// Java like api for typescript development

class Main extends java.Program{
    // INGORE //
    constructor(){super();}
    // INGORE //
    public main(): java.STATUS{
        console.log('Hello World');
        const e = new java.List<number>();
        e.add(1);
        e.add(2);
        return java.STATUS.FAILED;
    }
}



const main = new Main();