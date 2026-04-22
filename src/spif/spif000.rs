#[doc = "Register `SPIF000` reader"]
pub type R = crate::R<Spif000Spec>;
#[doc = "Register `SPIF000` writer"]
pub type W = crate::W<Spif000Spec>;
#[doc = "Field `PTENABLE` reader - PT_ENABLE"]
pub type PtenableR = crate::BitReader;
#[doc = "Field `PTENABLE` writer - PT_ENABLE"]
pub type PtenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTMBENABLE` reader - PT_MB_ENABLE"]
pub type PtmbenableR = crate::BitReader;
#[doc = "Field `PTMBENABLE` writer - PT_MB_ENABLE"]
pub type PtmbenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTERENABLE` reader - FILTER_ENABLE"]
pub type FilterenableR = crate::BitReader;
#[doc = "Field `FILTERENABLE` writer - FILTER_ENABLE"]
pub type FilterenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUXSEL` reader - MUX_SEL"]
pub type MuxselR = crate::BitReader;
#[doc = "Field `MUXSEL` writer - MUX_SEL"]
pub type MuxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RAMSEL` reader - RAM_SEL"]
pub type RamselR = crate::BitReader;
#[doc = "Field `RAMSEL` writer - RAM_SEL"]
pub type RamselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKMODE` reader - BLOCK_MODE"]
pub type BlockmodeR = crate::BitReader;
#[doc = "Field `BLOCKMODE` writer - BLOCK_MODE"]
pub type BlockmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBCLEAR` reader - WB_CLEAR"]
pub type WbclearR = crate::BitReader;
#[doc = "Field `WBCLEAR` writer - WB_CLEAR"]
pub type WbclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPIALLOW` reader - QPI_ALLOW"]
pub type QpiallowR = crate::BitReader;
#[doc = "Field `QPIALLOW` writer - QPI_ALLOW"]
pub type QpiallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVEMODE` reader - DRIVE_MODE"]
pub type DrivemodeR = crate::BitReader;
#[doc = "Field `DRIVEMODE` writer - DRIVE_MODE"]
pub type DrivemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBLOCKLATEST` reader - WBLOCK_LATEST"]
pub type WblocklatestR = crate::BitReader;
#[doc = "Field `WBLOCKLATEST` writer - WBLOCK_LATEST"]
pub type WblocklatestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSEL` reader - RST_SEL"]
pub type RstselR = crate::BitReader;
#[doc = "Field `RSTSEL` writer - RST_SEL"]
pub type RstselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTOUT` reader - RST_OUT"]
pub type RstoutR = crate::BitReader;
#[doc = "Field `RSTOUT` writer - RST_OUT"]
pub type RstoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTOE` reader - RST_OE"]
pub type RstoeR = crate::BitReader;
#[doc = "Field `RSTOE` writer - RST_OE"]
pub type RstoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` reader - SW_RST"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - SW_RST"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDB` reader - CK_DB"]
pub type CkdbR = crate::BitReader;
#[doc = "Field `CKDB` writer - CK_DB"]
pub type CkdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDB` reader - CS_DB"]
pub type CsdbR = crate::BitReader;
#[doc = "Field `CSDB` writer - CS_DB"]
pub type CsdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATDB` reader - DAT_DB"]
pub type DatdbR = crate::FieldReader;
#[doc = "Field `DATDB` writer - DAT_DB"]
pub type DatdbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCKFIFO` reader - LOCK_FIFO"]
pub type LockfifoR = crate::BitReader;
#[doc = "Field `LOCKFIFO` writer - LOCK_FIFO"]
pub type LockfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKRESET` reader - LOCK_RESET"]
pub type LockresetR = crate::BitReader;
#[doc = "Field `LOCKRESET` writer - LOCK_RESET"]
pub type LockresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBLOCKLEN` reader - WBLOCK_LEN"]
pub type WblocklenR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - PT_ENABLE"]
    #[inline(always)]
    pub fn ptenable(&self) -> PtenableR {
        PtenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PT_MB_ENABLE"]
    #[inline(always)]
    pub fn ptmbenable(&self) -> PtmbenableR {
        PtmbenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FILTER_ENABLE"]
    #[inline(always)]
    pub fn filterenable(&self) -> FilterenableR {
        FilterenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MUX_SEL"]
    #[inline(always)]
    pub fn muxsel(&self) -> MuxselR {
        MuxselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - RAM_SEL"]
    #[inline(always)]
    pub fn ramsel(&self) -> RamselR {
        RamselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BLOCK_MODE"]
    #[inline(always)]
    pub fn blockmode(&self) -> BlockmodeR {
        BlockmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WB_CLEAR"]
    #[inline(always)]
    pub fn wbclear(&self) -> WbclearR {
        WbclearR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - QPI_ALLOW"]
    #[inline(always)]
    pub fn qpiallow(&self) -> QpiallowR {
        QpiallowR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DRIVE_MODE"]
    #[inline(always)]
    pub fn drivemode(&self) -> DrivemodeR {
        DrivemodeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WBLOCK_LATEST"]
    #[inline(always)]
    pub fn wblocklatest(&self) -> WblocklatestR {
        WblocklatestR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RST_SEL"]
    #[inline(always)]
    pub fn rstsel(&self) -> RstselR {
        RstselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RST_OUT"]
    #[inline(always)]
    pub fn rstout(&self) -> RstoutR {
        RstoutR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RST_OE"]
    #[inline(always)]
    pub fn rstoe(&self) -> RstoeR {
        RstoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SW_RST"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CK_DB"]
    #[inline(always)]
    pub fn ckdb(&self) -> CkdbR {
        CkdbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CS_DB"]
    #[inline(always)]
    pub fn csdb(&self) -> CsdbR {
        CsdbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - DAT_DB"]
    #[inline(always)]
    pub fn datdb(&self) -> DatdbR {
        DatdbR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - LOCK_FIFO"]
    #[inline(always)]
    pub fn lockfifo(&self) -> LockfifoR {
        LockfifoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LOCK_RESET"]
    #[inline(always)]
    pub fn lockreset(&self) -> LockresetR {
        LockresetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - WBLOCK_LEN"]
    #[inline(always)]
    pub fn wblocklen(&self) -> WblocklenR {
        WblocklenR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PT_ENABLE"]
    #[inline(always)]
    pub fn ptenable(&mut self) -> PtenableW<Spif000Spec> {
        PtenableW::new(self, 0)
    }
    #[doc = "Bit 1 - PT_MB_ENABLE"]
    #[inline(always)]
    pub fn ptmbenable(&mut self) -> PtmbenableW<Spif000Spec> {
        PtmbenableW::new(self, 1)
    }
    #[doc = "Bit 2 - FILTER_ENABLE"]
    #[inline(always)]
    pub fn filterenable(&mut self) -> FilterenableW<Spif000Spec> {
        FilterenableW::new(self, 2)
    }
    #[doc = "Bit 3 - MUX_SEL"]
    #[inline(always)]
    pub fn muxsel(&mut self) -> MuxselW<Spif000Spec> {
        MuxselW::new(self, 3)
    }
    #[doc = "Bits 4:5 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<Spif000Spec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bit 6 - RAM_SEL"]
    #[inline(always)]
    pub fn ramsel(&mut self) -> RamselW<Spif000Spec> {
        RamselW::new(self, 6)
    }
    #[doc = "Bit 7 - BLOCK_MODE"]
    #[inline(always)]
    pub fn blockmode(&mut self) -> BlockmodeW<Spif000Spec> {
        BlockmodeW::new(self, 7)
    }
    #[doc = "Bit 8 - WB_CLEAR"]
    #[inline(always)]
    pub fn wbclear(&mut self) -> WbclearW<Spif000Spec> {
        WbclearW::new(self, 8)
    }
    #[doc = "Bit 9 - QPI_ALLOW"]
    #[inline(always)]
    pub fn qpiallow(&mut self) -> QpiallowW<Spif000Spec> {
        QpiallowW::new(self, 9)
    }
    #[doc = "Bit 10 - DRIVE_MODE"]
    #[inline(always)]
    pub fn drivemode(&mut self) -> DrivemodeW<Spif000Spec> {
        DrivemodeW::new(self, 10)
    }
    #[doc = "Bit 11 - WBLOCK_LATEST"]
    #[inline(always)]
    pub fn wblocklatest(&mut self) -> WblocklatestW<Spif000Spec> {
        WblocklatestW::new(self, 11)
    }
    #[doc = "Bit 12 - RST_SEL"]
    #[inline(always)]
    pub fn rstsel(&mut self) -> RstselW<Spif000Spec> {
        RstselW::new(self, 12)
    }
    #[doc = "Bit 13 - RST_OUT"]
    #[inline(always)]
    pub fn rstout(&mut self) -> RstoutW<Spif000Spec> {
        RstoutW::new(self, 13)
    }
    #[doc = "Bit 14 - RST_OE"]
    #[inline(always)]
    pub fn rstoe(&mut self) -> RstoeW<Spif000Spec> {
        RstoeW::new(self, 14)
    }
    #[doc = "Bit 15 - SW_RST"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<Spif000Spec> {
        SwrstW::new(self, 15)
    }
    #[doc = "Bit 16 - CK_DB"]
    #[inline(always)]
    pub fn ckdb(&mut self) -> CkdbW<Spif000Spec> {
        CkdbW::new(self, 16)
    }
    #[doc = "Bit 17 - CS_DB"]
    #[inline(always)]
    pub fn csdb(&mut self) -> CsdbW<Spif000Spec> {
        CsdbW::new(self, 17)
    }
    #[doc = "Bits 18:21 - DAT_DB"]
    #[inline(always)]
    pub fn datdb(&mut self) -> DatdbW<Spif000Spec> {
        DatdbW::new(self, 18)
    }
    #[doc = "Bit 22 - LOCK_FIFO"]
    #[inline(always)]
    pub fn lockfifo(&mut self) -> LockfifoW<Spif000Spec> {
        LockfifoW::new(self, 22)
    }
    #[doc = "Bit 23 - LOCK_RESET"]
    #[inline(always)]
    pub fn lockreset(&mut self) -> LockresetW<Spif000Spec> {
        LockresetW::new(self, 23)
    }
}
#[doc = "SPIF\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spif000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif000Spec;
impl crate::RegisterSpec for Spif000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif000::R`](R) reader structure"]
impl crate::Readable for Spif000Spec {}
#[doc = "`write(|w| ..)` method takes [`spif000::W`](W) writer structure"]
impl crate::Writable for Spif000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF000 to value 0"]
impl crate::Resettable for Spif000Spec {}
