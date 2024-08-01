package fr.kingfight.kingManageAPI.Objects.Status;

public class Status {
    private status plStatus;

    public Status() {
        this.plStatus = status.ENABLED;
    }

    public status getStatus() {
        return plStatus;
    }

    public void setStatus(status plStatus) {
        this.plStatus = plStatus;
    }
}

