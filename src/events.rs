///Trait given to a sink of events
pub trait EventSink {
    ///Process an event
    fn process_event<T>(&mut self, event: T);
}

///Trait given to data of events
pub trait EventData {}

///Trait describing events dispatchers, that are contained in the queue
pub trait EventDispatcher {
    ///Type of the sink this event is sent to
    type SinkT: EventSink;

    ///Type of the data this event is
    type DataT: EventData;

    ///send event to a sink
    fn dispatch(&mut self) {
        let data = self.data();
        self.target().process_event(data)
    }

    ///access to the sink this event targets
    fn target(&mut self) -> &mut Self::SinkT;

    ///access to the data internal of the event that get passed to the sink
    fn data(&mut self) -> Self::DataT;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::setup;
    use log::debug;

    ///basic state machine
    pub struct BasicSink {
        pub event_count: usize,
    }

    impl EventSink for BasicSink {
        fn process_event<T>(&mut self, _event: T) {
            debug!("Event received");
            self.event_count += 1;
        }
    }

    ///Basic event
    #[derive(Copy, Clone)]
    pub struct EmptyEventData {}

    ///Basic event dispatcher
    pub struct BasicEventDispatcher {
        pub(crate) sink: BasicSink,
    }

    impl EventData for EmptyEventData {}

    impl EventDispatcher for BasicEventDispatcher {
        type SinkT = BasicSink;
        type DataT = EmptyEventData;

        fn target(&mut self) -> &mut Self::SinkT {
            &mut self.sink
        }

        fn data(&mut self) -> Self::DataT {
            EmptyEventData {}
        }
    }
    #[test]
    fn test_dispatch() {
        setup();
        //setup a basic sink and event
        let sink = BasicSink { event_count: 0 };
        let mut event = BasicEventDispatcher { sink };

        event.dispatch();

        assert_eq!(event.sink.event_count, 1, "Event didn't increment sink");

        event.dispatch();

        assert_eq!(event.sink.event_count, 2, "Event didn't increment sink");
    }
}
