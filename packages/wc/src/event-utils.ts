/**
 * Generates Vue-compatible event handler types from a tuple of event constructors
 *
 * @template T - Tuple of event constructors that must have:
 *               1. A static `tag` property (kebab-case event name)
 *               2. An instance type representing the event object
 *
 * @example
 * ```ts
 *
 * type Handlers = [typeof IncrementEvent, typeof DecrementEvent, typeof ChangeValueEvent];
 *
 * // Equivalent to:
 * type Handlers = {
 *   onIncrement?: (event: IncrementEvent) => void;
 *   onDecrement?: (event: DecrementEvent) => void;
 *   onChangeValue?: (event: ChangeValueEvent) => void;
 * }
 * ```
 *
 * @remarks
 * - Automatically converts kebab-case tags to PascalCase handler names
 * - Preserves strong typing for event payloads
 * - Creates optional handlers for all events
 */
export type VueEventHandlers<T extends readonly (new (...args: any) => any)[]> = {
	/**
	 * Maps each event constructor to a Vue handler:
	 * 1. Extracts the static `tag` property (kebab-case event name)
	 * 2. Converts to PascalCase (my-event → MyEvent)
	 * 3. Prefixes with "on" (MyEvent → onMyEvent)
	 * 4. Creates optional handler with properly typed event parameter
	 *
	 * @param K - Event constructor from the tuple
	 * @returns Vue handler signature for the event
	 */
	[K in T[number] as `on${KebabToPascalCase<
		// Extract static 'tag' property from constructor
		K extends { tag: infer Tag extends string } ? Tag : never
	>}`]?: (
		// Instance type of the event constructor
		event: InstanceType<K>
	) => void;
};

/**
 * Converts kebab-case strings to camelCase
 *
 * @example
 * type T = KebabToCamelCase<'hello-world'>;  // 'helloWorld'
 */
type KebabToCamelCase<S extends string> = S extends `${infer First}-${infer Rest}`
	? `${First}${Capitalize<KebabToCamelCase<Rest>>}`
	: S;

/**
 * Converts kebab-case strings to PascalCase
 *
 * @example
 * type T = KebabToPascalCase<'hello-world'>;  // 'HelloWorld'
 */
type KebabToPascalCase<S extends string> = Capitalize<KebabToCamelCase<S>>;

/**
 * Converts tuple of event constructors to HTMLElementEventMap-compatible interface
 * @example
 * type MyEventMap = EventMapFrom<[typeof MyEvent]>;
 * // { 'my-event': MyEvent }
 */
export type EventMapFrom<T extends readonly (new (...args: any) => Event)[]> = {
	[K in T[number] as K extends { tag: infer Tag extends string } ? Tag : never]: InstanceType<K>;
};
