const editor = document.querySelector('.editor')
const editorContentInput = document.querySelector('.editor__content-input')
const editorContentPreviewContent = document.querySelector('.editor__content-preview-content')


window.onload = async function() {
  const res = await api.doGet(`/api/file/${editor.dataset.filename}`)
  const file = await res.json()
  editorContentInput.value = file.markdown
  editorContentPreviewContent.innerHTML = file.html
}

editorContentInput.addEventListener('input', async e => {
  const value = e.target.value
  await api.doPut('/api/file', {
    name: editor.dataset.filename,
    markdown: value,
  })
  const updated = await api.doGet(`/api/file/${editor.dataset.filename}`)
  const file = await updated.json()
  editorContentInput.value = file.markdown
  editorContentPreviewContent.innerHTML = file.html
})