import { Projection as ProjectionInterface } from '..';

export default class Projection<TState> implements ProjectionInterface<TState> {
  readonly id: Buffer;
  private _state: TState | null = null;

  constructor(id: Buffer) {
    this.id = id;
  }

  get state() {
    return this._state;
  }

  async start() {
    /**
     * To be implemented
     */
  }

  async stop() {
    /**
     * To be implemented
     */
  }
}
