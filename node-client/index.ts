type Event<TType extends string = string, TData = unknown, TMeta = unknown> = {
  id: Buffer;
  type: TType;
  aggregateId: Buffer;
  aggregateVersion: number;
  data: TData;
  meta: TMeta;
  timestamp: Date;
}

interface Aggregate<TState> {
  id: Buffer;
  version: number;
  createEvent(event: Event<string, TState>): Promise<void>;
  fold(): Promise<TState>;
}

interface Projection<TState> {
  id: Buffer;
  state: TState;
  start(): Promise<void>;
  stop(): Promise<void>;
}

interface Connection {
  loadAggregate<TState>(
    id: Buffer;
    options?: {
      eventHandlers?: Record<string, (event: Event, state: TState) => Promise<TState> | TState>;
      onInitializeState?: () => Promise<TState> | TState;
      onShouldTakeSnapshot?: (event: Event, state: TState) => Promise<boolean>;
      onLoadSnapshot?: (aggregateId: Buffer) => Promise<{
        aggregateVersion: number;
        state: TState;
      }>;
      onTakeSnapshot?: (aggregateId: Buffer, aggregateVersion: number, state: TState) => Promise<void>;
    },
  ): Promise<Aggregate<TState>>,
  initializeProjection<TState>(
    id: Buffer;
    options?: {
      eventHandlers?: Record<string, (event: Event, state: TState) => Promise<TState> | TState>;
      onInitializeState?: () => Promise<TState> | TState;
      autoStart?: true;
    },
  ): Promise<Projection<TState>>;
}

interface Arque {
  connect(options: {
    hostname?: string;
    port?: number;
    servers?: [
      {
        hostname?: string;
        port?: number;
      }
    ]
  }): Promise<Connection>,
}

interface Page<TNode> {
  edges: [{
    node: TNode;
    cursor: Buffer;
  }];
  pageInfo: {
    hasNextPage: boolean;
    endCursor: null | Buffer;
  };
}

interface Driver {
  connect(params: {
    hostname?: string;
    port?: number;
    servers?: [
      {
        hostname?: string;
        port?: number;
      }
    ]
  }): Promise<void>;
  createEvent(params: {
    type: string;
    aggregateId: Buffer;
    aggregateVersion: number;
    data: Record<string, any>;
    meta?: Record<string, any>;
  }): Promise<void>;
  readAggregateEvents(first?: number, after?: Buffer, params: {
    aggregateId: Buffer;
  }): Promise<Page<Event>>;
  readEvents(first?: number, after?: Buffer, params: {
    types: string[]
  }): Promise<Page<Event>>;
  subscribe(): Promise<Stream>;
}
