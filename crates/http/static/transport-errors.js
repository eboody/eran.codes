import { mergePatch } from '/static/datastar.js';

if (window.__transportErrorsBound !== true) {
  window.__transportErrorsBound = true;

  const clearTransportState = ({ clearConnection = false } = {}) => {
    const patch = {
      transportErrorSource: '',
      transportErrorKind: '',
      transportErrorTitle: '',
      transportErrorMessage: '',
      transportErrorStatus: 0,
      transportRetrying: false,
    };

    if (clearConnection) {
      patch.sseConnected = false;
    }

    mergePatch(patch);
  };

  const setTransportState = ({
    source,
    kind,
    title,
    message,
    status = 0,
    retrying = false,
    disconnect = false,
  }) => {
    const patch = {
      transportErrorSource: source,
      transportErrorKind: kind,
      transportErrorTitle: title,
      transportErrorMessage: message,
      transportErrorStatus: status,
      transportRetrying: retrying,
    };

    if (disconnect) {
      patch.sseConnected = false;
    }

    mergePatch(patch);
  };

  const isBodyRequest = (element) => element === document.body;

  document.addEventListener('datastar-fetch', (event) => {
    const detail = event?.detail;
    if (!detail) return;

    const sourceElement = detail.el;
    const isSseRequest = isBodyRequest(sourceElement);

    switch (detail.type) {
      case 'started':
        if (!isSseRequest) {
          clearTransportState();
        }
        break;

      case 'finished':
        if (isSseRequest) {
          mergePatch({
            sseConnected: false,
            transportRetrying: false,
          });
        }
        break;

      case 'retrying': {
        const message = typeof detail.argsRaw?.message === 'string'
          && detail.argsRaw.message.length > 0
          ? detail.argsRaw.message
          : isSseRequest
            ? 'Connection lost. Retrying.'
            : 'Request failed. Retrying.';
        setTransportState({
          source: 'client',
          kind: 'retrying',
          title: isSseRequest ? 'Connection lost' : 'Request retrying',
          message,
          retrying: true,
          disconnect: isSseRequest,
        });
        break;
      }

      case 'retries-failed':
        setTransportState({
          source: 'client',
          kind: 'network',
          title: isSseRequest ? 'Disconnected' : 'Request failed',
          message: isSseRequest
            ? 'Connection retry limit reached.'
            : 'Request retry limit reached.',
          disconnect: isSseRequest,
        });
        break;

      case 'error': {
        const status = Number(detail.argsRaw?.status ?? 0);
        setTransportState({
          source: 'client',
          kind: status === 401 ? 'auth' : 'network',
          title: status > 0 ? 'Request failed' : 'Network error',
          message: status > 0
            ? `Request failed with status ${status}.`
            : 'The request could not be completed.',
          status,
          disconnect: isSseRequest,
        });
        break;
      }

      default:
        break;
    }
  });

  document.addEventListener('datastar-signal-patch', (event) => {
    if (event?.detail?.sseConnected === true) {
      clearTransportState();
    }
  });
}
