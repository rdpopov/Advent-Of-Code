package main

// import "fmt"
import (
	"fmt"
	"list"
)


func Part1(maxmarble int,players int) int{

    var cyclical_list list.MapList
    cyclical_list.Add(0,0,0)

    var player_scores = make([]int, players)

    var insert_key int = 0

        for mar:= 1; mar < maxmarble; mar ++ {
            cyclical_list.PrintFromKey(0,insert_key)
            if (mar) % 23 == 0  {
                var to_remove = cyclical_list.GetRelative(insert_key,-7)
                insert_key = cyclical_list.GetRelative(to_remove,1)
                var marble_worth = to_remove + mar
                // if marble_worth == maxmarble {
                //     break
                // }
                player_scores[mar%players] += marble_worth
                // if player_scores[mar%players] >= 800 {
                //     fmt.Println(player_scores[mar%players],mar,insert_key,marble_worth,to_remove)
                // }
                cyclical_list.Remove(to_remove)
                cyclical_list.Remove(mar)
            } else {
                cyclical_list.Add(mar,insert_key,1)
                insert_key = cyclical_list.GetRelative(insert_key+1,0)
            }

    }
    var max int = player_scores[0]
    for _,x := range  player_scores {
        if x >= max {
            max = x
        }
    }
    return max
}

func main() {
    // var cyclical_list list.MapList
    fmt.Printf("max score %d %d \n",32,Part1(28,9))
    // fmt.Printf("max score %d %d \n",8317,Part1(1618,11))
}
