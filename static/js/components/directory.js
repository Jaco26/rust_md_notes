import DirectoryItems from './directory-items.js'

export default {
  functional: true,
  name: 'Directory',
  props: {
    child_dirs: Array,
    dirsOpenState: Object,
    id: String,
    files: Array,
    name: String,
  },
  render(h, ctx) {
    const { props, listeners } = ctx
    const children = [
      h(
        'button',
        {
          class: 'directory__name',
          on: {
            click: () => {
              listeners.toggleOpenState({
                id: props.id,
                isOpen: !props.dirsOpenState[props.id]
              })
            }
          }
        },
        props.name
      ),
      props.dirsOpenState[props.id] && h(
        DirectoryItems,
        {
          on: {
            toggleOpenState: listeners.toggleOpenState,
          },
          props: {
            child_dirs: props.child_dirs,
            files: props.files,
            dirsOpenState: props.dirsOpenState
          }
        },
      )
    ]

    return h(
      'div',
      { class: 'directory' },
      children
    )
  }
}

