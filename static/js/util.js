

const doFetch = async (url, { method, body = {}, headers = {} } = {}) => {
  const options = {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...headers
    }
  }

  if (method === 'post' || method === 'put' && body) {
    options.body = JSON.stringify(body)
  }

  const response = await fetch(encodeURI(url), options)

  return await response.json()
}

export const api = {
  doGet: (url, headers) => doFetch(url, { method: 'get', headers }),
  doDelete: (url, headers) => doFetch(url, { method: 'delete', headers }),
  doPost: (url, body, headers) => doFetch(url, { method: 'post', body, headers }),
  doPut: (url, body, headers) => doFetch(url, { method: 'put', body, headers }),
}