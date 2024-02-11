use crate::register_metric;

use prometheus::Gauge;

register_metric!(node_load1, Gauge, "1m load average.", |state, m| {
    let g: Gauge = m.try_into()?;
    g.set(state.load1().into());
    Ok(())
});

#[cfg(test)]
mod tests_load1 {
    use crate::tests::{dump_with_state, MockSystemState};
    use crate::Result;

    #[test]
    fn valid() -> Result<()> {
        let mut state = MockSystemState::default();
        state.load1 = 0.5;
        let (m, _) = dump_with_state(&state)?;
        assert!(m.contains("node_load1 0.5"), "Metrics: <{}>", m);
        Ok(())
    }
}

register_metric!(node_load15, Gauge, "15m load average.", |state, m| {
    let g: Gauge = m.try_into()?;
    g.set(state.load15().into());
    Ok(())
});

#[cfg(test)]
mod tests_load15 {
    use crate::tests::{dump_with_state, MockSystemState};
    use crate::Result;

    #[test]
    fn valid() -> Result<()> {
        let mut state = MockSystemState::default();
        state.load15 = 0.5;
        let (m, _) = dump_with_state(&state)?;
        assert!(m.contains("node_load15 0.5"), "Metrics: <{}>", m);
        Ok(())
    }
}

register_metric!(node_load5, Gauge, "5m load average.", |state, m| {
    let g: Gauge = m.try_into()?;
    g.set(state.load5().into());
    Ok(())
});

#[cfg(test)]
mod tests_load5 {
    use crate::tests::{dump_with_state, MockSystemState};
    use crate::Result;

    #[test]
    fn valid() -> Result<()> {
        let mut state = MockSystemState::default();
        state.load5 = 0.5;
        let (m, _) = dump_with_state(&state)?;
        assert!(m.contains("node_load5 0.5"), "Metrics: <{}>", m);
        Ok(())
    }
}
