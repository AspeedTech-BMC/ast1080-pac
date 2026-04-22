#[doc = "Register `SPIPF07C` reader"]
pub type R = crate::R<Spipf07cSpec>;
#[doc = "Register `SPIPF07C` writer"]
pub type W = crate::W<Spipf07cSpec>;
#[doc = "Field `WrDisOfSPIPF002` reader - Write Disable of hlinkSPIPF00"]
pub type WrDisOfSpipf002R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF002` writer - Write Disable of hlinkSPIPF00"]
pub type WrDisOfSpipf002W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF001` reader - Write Disable of hlinkSPIPF00"]
pub type WrDisOfSpipf001R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF001` writer - Write Disable of hlinkSPIPF00"]
pub type WrDisOfSpipf001W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF00` reader - Write Disable of hlinkSPIPF00"]
pub type WrDisOfSpipf00R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF00` writer - Write Disable of hlinkSPIPF00"]
pub type WrDisOfSpipf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF011` reader - Write Disable of hlinkSPIPF01"]
pub type WrDisOfSpipf011R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF011` writer - Write Disable of hlinkSPIPF01"]
pub type WrDisOfSpipf011W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF01` reader - Write Disable of hlinkSPIPF01"]
pub type WrDisOfSpipf01R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF01` writer - Write Disable of hlinkSPIPF01"]
pub type WrDisOfSpipf01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF02` reader - Write Disable of hlinkSPIPF02"]
pub type WrDisOfSpipf02R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF02` writer - Write Disable of hlinkSPIPF02"]
pub type WrDisOfSpipf02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `WrDisOfSPIPFW` reader - Write Disable of hlinkSPIPFW"]
pub type WrDisOfSpipfwR = crate::BitReader;
#[doc = "Field `WrDisOfSPIPFW` writer - Write Disable of hlinkSPIPFW"]
pub type WrDisOfSpipfwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPFR` reader - Write Disable of hlinkSPIPFR"]
pub type WrDisOfSpipfrR = crate::BitReader;
#[doc = "Field `WrDisOfSPIPFR` writer - Write Disable of hlinkSPIPFR"]
pub type WrDisOfSpipfrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Disable of hlinkSPIPF00"]
    #[inline(always)]
    pub fn wr_dis_of_spipf002(&self) -> WrDisOfSpipf002R {
        WrDisOfSpipf002R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Disable of hlinkSPIPF00"]
    #[inline(always)]
    pub fn wr_dis_of_spipf001(&self) -> WrDisOfSpipf001R {
        WrDisOfSpipf001R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Disable of hlinkSPIPF00"]
    #[inline(always)]
    pub fn wr_dis_of_spipf00(&self) -> WrDisOfSpipf00R {
        WrDisOfSpipf00R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Disable of hlinkSPIPF01"]
    #[inline(always)]
    pub fn wr_dis_of_spipf011(&self) -> WrDisOfSpipf011R {
        WrDisOfSpipf011R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Disable of hlinkSPIPF01"]
    #[inline(always)]
    pub fn wr_dis_of_spipf01(&self) -> WrDisOfSpipf01R {
        WrDisOfSpipf01R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Disable of hlinkSPIPF02"]
    #[inline(always)]
    pub fn wr_dis_of_spipf02(&self) -> WrDisOfSpipf02R {
        WrDisOfSpipf02R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:29 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 9) & 0x001f_ffff)
    }
    #[doc = "Bit 30 - Write Disable of hlinkSPIPFW"]
    #[inline(always)]
    pub fn wr_dis_of_spipfw(&self) -> WrDisOfSpipfwR {
        WrDisOfSpipfwR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Disable of hlinkSPIPFR"]
    #[inline(always)]
    pub fn wr_dis_of_spipfr(&self) -> WrDisOfSpipfrR {
        WrDisOfSpipfrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Disable of hlinkSPIPF00"]
    #[inline(always)]
    pub fn wr_dis_of_spipf002(&mut self) -> WrDisOfSpipf002W<Spipf07cSpec> {
        WrDisOfSpipf002W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Disable of hlinkSPIPF00"]
    #[inline(always)]
    pub fn wr_dis_of_spipf001(&mut self) -> WrDisOfSpipf001W<Spipf07cSpec> {
        WrDisOfSpipf001W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Disable of hlinkSPIPF00"]
    #[inline(always)]
    pub fn wr_dis_of_spipf00(&mut self) -> WrDisOfSpipf00W<Spipf07cSpec> {
        WrDisOfSpipf00W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Spipf07cSpec> {
        Reserved1W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Disable of hlinkSPIPF01"]
    #[inline(always)]
    pub fn wr_dis_of_spipf011(&mut self) -> WrDisOfSpipf011W<Spipf07cSpec> {
        WrDisOfSpipf011W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Disable of hlinkSPIPF01"]
    #[inline(always)]
    pub fn wr_dis_of_spipf01(&mut self) -> WrDisOfSpipf01W<Spipf07cSpec> {
        WrDisOfSpipf01W::new(self, 5)
    }
    #[doc = "Bit 8 - Write Disable of hlinkSPIPF02"]
    #[inline(always)]
    pub fn wr_dis_of_spipf02(&mut self) -> WrDisOfSpipf02W<Spipf07cSpec> {
        WrDisOfSpipf02W::new(self, 8)
    }
    #[doc = "Bit 30 - Write Disable of hlinkSPIPFW"]
    #[inline(always)]
    pub fn wr_dis_of_spipfw(&mut self) -> WrDisOfSpipfwW<Spipf07cSpec> {
        WrDisOfSpipfwW::new(self, 30)
    }
    #[doc = "Bit 31 - Write Disable of hlinkSPIPFR"]
    #[inline(always)]
    pub fn wr_dis_of_spipfr(&mut self) -> WrDisOfSpipfrW<Spipf07cSpec> {
        WrDisOfSpipfrW::new(self, 31)
    }
}
#[doc = "Write Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf07cSpec;
impl crate::RegisterSpec for Spipf07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf07c::R`](R) reader structure"]
impl crate::Readable for Spipf07cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf07c::W`](W) writer structure"]
impl crate::Writable for Spipf07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF07C to value 0"]
impl crate::Resettable for Spipf07cSpec {}
