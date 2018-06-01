namespace input {
/////////////////////////////////////////////////////////////////////////////

export namespace Key
{
	export class Event extends egret.Event
	{
		public static readonly KEY_UP   = "input.Key.Event.KEY_UP";
		public static readonly KEY_DOWN = "input.Key.Event.KEY_DOWN";

		public constructor(
			public  key     : Readonly<Key>,
			private keyboard: Readonly<KeyBoard>,
			type      : string,
			bubbles   : boolean = false,
			cancelable: boolean = false)
		{
			super(type, bubbles, cancelable);
		}

		isDown(key: Key): boolean
		{
			return this.keyboard.isKeyDown(key);
		}

		isUp(key: Key): boolean
		{
			return this.keyboard.isKeyUp(key);
		}

		get keys(): ReadonlySet<Key>
		{
			return this.keys;
		}
	}
}

export class KeyBoard extends egret.EventDispatcher
{
	private keys : Set<Key> = new Set<Key>();

    public constructor()
	{
		super();
		const self = this;

		document.onkeydown = function(event)
		{
            const e = event || window.event || arguments.callee.caller.arguments[0];
			const c = e.keyCode;
			if (Key.Enums.has(c) === false || self.keys.has(c) === true)
				return;

			self.keys.add(c);
			const k = new Key.Event(c, self, Key.Event.KEY_DOWN, true, true);
			self.dispatchEvent(k);

			// console.debug("On Key Down " + c);
		}

        document.onkeyup = function(event)
		{
            const e = event || window.event || arguments.callee.caller.arguments[0];
			const c = e.keyCode;
			if (Key.Enums.has(c) === false || self.keys.has(c) === false)
				return;

			self.keys.delete(c);
			const k = new Key.Event(c, self, Key.Event.KEY_UP, true, true);
			self.dispatchEvent(k);

			// console.debug("On Key Up " + c);
        }

		// TODO: with mouse.
		// document.onmousedown = function(event) {
		// 	self.keys.clear();
		// }
	}

	isKeyDown(key: Key): boolean
	{
		return this.keys.has(key);
	}

	isKeyUp(key: Key): boolean
	{
		return this.isKeyDown(key) === false;
	}
}

/////////////////////////////////////////////////////////////////////////////
}