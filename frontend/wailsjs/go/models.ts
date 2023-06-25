export namespace main {
	
	export class Result {
	    name: string;
	    status: boolean;
	
	    static createFrom(source: any = {}) {
	        return new Result(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.status = source["status"];
	    }
	}

}

export namespace model {
	
	export class QueryResult {
	    episode: string;
	    id: string;
	    img: string;
	    title: string;
	    sub_title: string;
	    type: string;
	    url: string;
	    year: string;
	
	    static createFrom(source: any = {}) {
	        return new QueryResult(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.episode = source["episode"];
	        this.id = source["id"];
	        this.img = source["img"];
	        this.title = source["title"];
	        this.sub_title = source["sub_title"];
	        this.type = source["type"];
	        this.url = source["url"];
	        this.year = source["year"];
	    }
	}

}

