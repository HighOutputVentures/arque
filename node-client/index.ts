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
  createEvent(): Promise<void>;
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