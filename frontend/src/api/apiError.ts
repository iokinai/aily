export enum ErrorType {
  NotFound = 0,
  InvalidSession = 1,
  ProxyError = 2,
  Unknown = 3,
  ServerConnection = 4,
}

export type NotFoundInner = "artwork";

export type ProxyErrorInner =
  | { could_not_send_request: { status: number | null } }
  | { request_failed: { status: number } }
  | "unable_to_read_response";

export type HttpErrorBody =
  | { error_type: ErrorType.NotFound; inner: NotFoundInner }
  | { error_type: ErrorType.InvalidSession; inner: null | string }
  | { error_type: ErrorType.ProxyError; inner: ProxyErrorInner }
  | { error_type: ErrorType.Unknown; inner: null | string }
  | { error_type: ErrorType.ServerConnection; inner: null };

export function errorToStringRu(responseBody: HttpErrorBody): string | null {
  switch (responseBody.error_type) {
    case ErrorType.NotFound:
      if (responseBody.inner === "artwork") {
        return "арт не найден";
      }
      return null;

    case ErrorType.ProxyError:
      const proxyErr = responseBody.inner;

      if (typeof proxyErr === "object" && proxyErr !== null) {
        if ("could_not_send_request" in proxyErr) {
          return `Ошибка отправки, статус: ${proxyErr.could_not_send_request.status}`;
        } else if ("request_failed" in proxyErr) {
          return `Ошибка запроса, статус: ${proxyErr.request_failed.status}`;
        }
      } else if (proxyErr === "unable_to_read_response") {
        return "Не удалось прочитать ответ прокси.";
      }
      return null;

    case ErrorType.InvalidSession:
      return "Сессия недействительна";

    case ErrorType.ServerConnection:
      return "Не удалось подключиться к серверу";

    case ErrorType.Unknown:
    default:
      return "Неизвестная ошибка";
  }
}
