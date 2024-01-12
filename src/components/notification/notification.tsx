import { component$ } from '@builder.io/qwik'

export type NotificationProps =
    | { state: 'none'; title: string; msg: string }
    // | { state: 'notif'; title: string }
    | { state: 'info'; title: string; msg: string }
    | { state: 'warning'; title: string; msg: string }
    | { state: 'error'; title: string; msg: string; additionalInfo: string }

const notifColor = (state: 'none' | 'info' | 'warning' | 'error') => {
    switch (state) {
        case 'none':
            return 'border-neutral-500'
        case 'info':
            return 'border-lime-500'
        case 'warning':
            return 'border-yellow-500'
        case 'error':
            return 'border-red-500'
    }
}

export const Notification = component$<NotificationProps>((props) => {
    const c = notifColor(props.state)

    return (
        <div
            class={`absolute right-6 top-6 flex h-auto w-64 flex-col border-l-2 p-2 pl-4 ${c}`}
        >
            <p class="text-lg font-semibold">{props.title}</p>
            <p>{props.msg}</p>
            {props.state == 'error' ? (
                <p class="text-neutral-500">{props.additionalInfo}</p>
            ) : (
                <></>
            )}
        </div>
    )
})
