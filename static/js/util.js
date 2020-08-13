

const doFetch = async (uri, { method, body = {}, headers = {} } = {}) => {
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

  const response = await fetch(encodeURI(`/api${uri}`), options)

  const rv = { data: null, status: response.status, statusText: response.statusText }

  try {
    rv.data = await response.json()
  } catch (error) { }

  return rv
}



export const api = {
  doGet: (uri, headers) => doFetch(uri, { method: 'get', headers }),
  doDelete: (uri, headers) => doFetch(uri, { method: 'delete', headers }),
  doPost: (uri, body, headers) => doFetch(uri, { method: 'post', body, headers }),
  doPut: (uri, body, headers) => doFetch(uri, { method: 'put', body, headers }),
}

export const buildFileTreeRecursively = (
  accum,
  directoryDictionary,
  currNode
) => {
  accum[currNode.id] = currNode
  accum[currNode.id].child_dirs = (currNode.child_dirs || []).map(child => (
    buildFileTreeRecursively(
      {},
      directoryDictionary,
      directoryDictionary[child.id]
    )
  ))
  return accum
}