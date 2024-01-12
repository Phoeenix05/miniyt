import { component$ } from '@builder.io/qwik'
import { useNavigate } from '@builder.io/qwik-city'

export type ThumbnailProps = {
    videoId: string
    style: 'full' | 'compact' | 'minimal' | 'channelPage'
}

export const Thumbnail = component$<ThumbnailProps>((props) => {
    const { videoId, style } = props
    const nav = useNavigate()

    // I'll need to load the data for these from invidious
    const thumbnailSrc: string = ''
    const title: string = ''
    const channelId: string = ''

    return (
        <span
            onClick$={async () => {
                await nav(`/video-${videoId}`)
            }}
        >
            <img
                src={thumbnailSrc}
                alt={`image not found: ${thumbnailSrc}`}
                width={200}
                height={112}
            />
            <p class="text-lg font-semibold">{title}</p>
            {style == 'channelPage' ? (
                <></>
            ) : (
                <a href={`/channel-${channelId}`}>
                    something with channel info: {channelId}
                </a>
            )}
        </span>
    )
})
