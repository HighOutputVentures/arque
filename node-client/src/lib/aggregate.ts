import { Aggregate as AggregateInterface, Event } from '..';

export default class Aggregate<TState> implements AggregateInterface<TState> {
  readonly id: Buffer;
  private _state: TState | null = null;
  private _version = 0;

  constructor(id: Buffer) {
    this.id = id;
  }

  get version() {
    return this._version;
  }

  async fold() {
    return this._state;
  }

  async createEvent(_event: Event) {
    /**
     * To be implemented
     */
  }
}
