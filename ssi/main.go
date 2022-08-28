package main

import (
	"context"
	"fmt"
	"time"

	"github.com/jackc/pgtype"
	"github.com/jackc/pgx/v4"
	"github.com/jackc/pgx/v4/pgxpool"
)

func main() {
	conn, err := pgxpool.Connect(context.Background(), "postgres://postgres:dalong@localhost:5432/postgres?sslmode=disable")
	handleErr(err)
	defer conn.Close()
	st := make(chan string)
	go t1(conn, st)
	go t2(conn, st)
	time.Sleep(3 * time.Second)

}
func t2(pool *pgxpool.Pool, signal chan string) {
	ctx := context.Background()
	tx, err := pool.BeginTx(ctx, pgx.TxOptions{
		IsoLevel: pgx.Serializable,
	})
	handleErr(err)
	val := pgtype.Int8{}

	// t1 has read id=1, it's time we read id=2 and update to id=1
	<-signal
	err = tx.QueryRow(ctx, "select count(*) from views").Scan(&val)
	handleErr(err)
	fmt.Printf("t2 read value %v\n", val)
	// _, err = tx.Exec(ctx, "update views set value=$1 where id=$2", val.String, "1")
	_, err = tx.Exec(ctx, "insert into views values('3','a')")
	if err != nil {
		fmt.Printf("t2 exec error %v\n", err)
		panic("")
	}
	// notify t1 to update id=2 to perform a skew write
	signal <- ""
	fmt.Printf("t2 update to %v\n", val)
	handleErr(tx.Commit(ctx))
}

func t1(pool *pgxpool.Pool, signal chan string) {
	ctx := context.Background()
	tx, err := pool.BeginTx(ctx, pgx.TxOptions{
		IsoLevel: pgx.Serializable,
	})
	handleErr(err)
	// val := pgtype.Text{}
	val := pgtype.Int8{}
	// err = tx.QueryRow(ctx, "select value from views where id=$1", "1").Scan(&val)
	err = tx.QueryRow(ctx, "select count(*) from views").Scan(&val)
	handleErr(err)
	fmt.Printf("t1 read val %v\n", val)
	signal <- ""
	<-signal
	err = tx.QueryRow(ctx, "select count(*) from views").Scan(&val)
	handleErr(err)
	fmt.Printf("t1 read val %v\n", val)
	_, err = tx.Exec(ctx, "insert into views values('4','b')")
	if err != nil {
		fmt.Printf("t1 exec error %v", err)
		panic("")
	}
	tx.Commit(ctx)
}

func handleErr(err error) {
	if err != nil {
		panic(err)
	}
}
