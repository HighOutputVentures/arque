import {
  Arque as ArqueInterface,
  ArqueConnectionOptions,
  InitializeProjectionOptions,
  LoadAggregateOptions,
} from '..';
import Aggregate from './aggregate';
import Projection from './projection';

export default class Arque implements ArqueInterface {
  async connect(_options: ArqueConnectionOptions) {
    return {
      loadAggregate: this._loadAggregate,
      initializeProjection: this._initializeProjection,
    };
  }

  private async _loadAggregate<TState>(
    id: Buffer,
    _options: LoadAggregateOptions<TState>,
  ) {
    const aggregate = new Aggregate<TState>(id);
    await aggregate.fold();

    return aggregate;
  }

  private async _initializeProjection<TState>(
    id: Buffer,
    options: InitializeProjectionOptions<TState>,
  ) {
    const projection = new Projection<TState>(id);
    if (options.autoStart) {
      await projection.start();
    }

    return projection;
  }
}
