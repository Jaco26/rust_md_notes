import Directory from './directory.js'

export default {
  functional: true,
  name: 'DirectoryItems',
  props: {
    child_dirs: Array,
    files: Array,
  },
  render(h, ctx) {
    return h(
      'div',
      { class: 'dir-items' },
      [
        h(
          'ul',
          { class: 'dir-items__files' },
          ctx.props.files.map(file => (
            h(
              'li',
              {
                on: { click: () => console.log(`You clicked this file ${file.name}`) }
              },
              file.name
            )
          ))
        ),
        h(
          'ul',
          { class: 'dir-items__child-dirs' },
          ctx.props.child_dirs.map(childDir => (
            h(
              Directory,
              {
                props: { name: childDir.name, files: childDir.files, child_dirs: childDir.child_dirs },
              }
            )
          ))
        )
      ]
    )
  }
}
