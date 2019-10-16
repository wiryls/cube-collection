namespace utils {
/////////////////////////////////////////////////////////////////////////////

import Scene = egret.DisplayObject;

export class Director
{
    public  main: Main;
    private stack = new Array<Scene>();

    private constructor() { }
    private static instance_: Director;
    public static get instance()
    {
        return this.instance_ || (this.instance_ = new this());
    }

    public push(scene: Scene): void
    {
        if (scene && this.main) {
            this.stack.push(scene);
            this.main.addChild(scene);
        }
    }

    public pop(): void
    {
        if (this.stack.length > 0) {
            const top = this.stack[this.stack.length - 1];
            this.stack.pop();
            if (top.parent === this.main)
                this.main.removeChild(top);
        }
    }

    public replace(scene: Scene): void
    {
        if (this.stack.length > 0 && scene) {
            this.pop();
            this.push(scene);
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
}