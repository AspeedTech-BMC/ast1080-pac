#[doc = "Register `SPIPF07C` reader"]
pub type R = crate::R<Spipf07cSpec>;
#[doc = "Register `SPIPF07C` writer"]
pub type W = crate::W<Spipf07cSpec>;
#[doc = "Field `WrDisOfSPIPF000` reader - Write Disable of SPIPF000"]
pub type WrDisOfSpipf000R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF000` writer - Write Disable of SPIPF000"]
pub type WrDisOfSpipf000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF004` reader - Write Disable of SPIPF004"]
pub type WrDisOfSpipf004R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF004` writer - Write Disable of SPIPF004"]
pub type WrDisOfSpipf004W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF008` reader - Write Disable of SPIPF008"]
pub type WrDisOfSpipf008R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF008` writer - Write Disable of SPIPF008"]
pub type WrDisOfSpipf008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF010` reader - Write Disable of SPIPF010"]
pub type WrDisOfSpipf010R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF010` writer - Write Disable of SPIPF010"]
pub type WrDisOfSpipf010W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF014` reader - Write Disable of SPIPF014"]
pub type WrDisOfSpipf014R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF014` writer - Write Disable of SPIPF014"]
pub type WrDisOfSpipf014W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPF020` reader - Write Disable of SPIPF020"]
pub type WrDisOfSpipf020R = crate::BitReader;
#[doc = "Field `WrDisOfSPIPF020` writer - Write Disable of SPIPF020"]
pub type WrDisOfSpipf020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `WrDisOfSPIPFWA` reader - Write Disable of SPIPFWA"]
pub type WrDisOfSpipfwaR = crate::BitReader;
#[doc = "Field `WrDisOfSPIPFWA` writer - Write Disable of SPIPFWA"]
pub type WrDisOfSpipfwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfSPIPFRA` reader - Write Disable of SPIPFRA"]
pub type WrDisOfSpipfraR = crate::BitReader;
#[doc = "Field `WrDisOfSPIPFRA` writer - Write Disable of SPIPFRA"]
pub type WrDisOfSpipfraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Disable of SPIPF000"]
    #[inline(always)]
    pub fn wr_dis_of_spipf000(&self) -> WrDisOfSpipf000R {
        WrDisOfSpipf000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Disable of SPIPF004"]
    #[inline(always)]
    pub fn wr_dis_of_spipf004(&self) -> WrDisOfSpipf004R {
        WrDisOfSpipf004R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Disable of SPIPF008"]
    #[inline(always)]
    pub fn wr_dis_of_spipf008(&self) -> WrDisOfSpipf008R {
        WrDisOfSpipf008R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Disable of SPIPF010"]
    #[inline(always)]
    pub fn wr_dis_of_spipf010(&self) -> WrDisOfSpipf010R {
        WrDisOfSpipf010R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Disable of SPIPF014"]
    #[inline(always)]
    pub fn wr_dis_of_spipf014(&self) -> WrDisOfSpipf014R {
        WrDisOfSpipf014R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Disable of SPIPF020"]
    #[inline(always)]
    pub fn wr_dis_of_spipf020(&self) -> WrDisOfSpipf020R {
        WrDisOfSpipf020R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:29 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 9) & 0x001f_ffff)
    }
    #[doc = "Bit 30 - Write Disable of SPIPFWA"]
    #[inline(always)]
    pub fn wr_dis_of_spipfwa(&self) -> WrDisOfSpipfwaR {
        WrDisOfSpipfwaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Disable of SPIPFRA"]
    #[inline(always)]
    pub fn wr_dis_of_spipfra(&self) -> WrDisOfSpipfraR {
        WrDisOfSpipfraR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Disable of SPIPF000"]
    #[inline(always)]
    pub fn wr_dis_of_spipf000(&mut self) -> WrDisOfSpipf000W<Spipf07cSpec> {
        WrDisOfSpipf000W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Disable of SPIPF004"]
    #[inline(always)]
    pub fn wr_dis_of_spipf004(&mut self) -> WrDisOfSpipf004W<Spipf07cSpec> {
        WrDisOfSpipf004W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Disable of SPIPF008"]
    #[inline(always)]
    pub fn wr_dis_of_spipf008(&mut self) -> WrDisOfSpipf008W<Spipf07cSpec> {
        WrDisOfSpipf008W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Spipf07cSpec> {
        Reserved1W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Disable of SPIPF010"]
    #[inline(always)]
    pub fn wr_dis_of_spipf010(&mut self) -> WrDisOfSpipf010W<Spipf07cSpec> {
        WrDisOfSpipf010W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Disable of SPIPF014"]
    #[inline(always)]
    pub fn wr_dis_of_spipf014(&mut self) -> WrDisOfSpipf014W<Spipf07cSpec> {
        WrDisOfSpipf014W::new(self, 5)
    }
    #[doc = "Bit 8 - Write Disable of SPIPF020"]
    #[inline(always)]
    pub fn wr_dis_of_spipf020(&mut self) -> WrDisOfSpipf020W<Spipf07cSpec> {
        WrDisOfSpipf020W::new(self, 8)
    }
    #[doc = "Bit 30 - Write Disable of SPIPFWA"]
    #[inline(always)]
    pub fn wr_dis_of_spipfwa(&mut self) -> WrDisOfSpipfwaW<Spipf07cSpec> {
        WrDisOfSpipfwaW::new(self, 30)
    }
    #[doc = "Bit 31 - Write Disable of SPIPFRA"]
    #[inline(always)]
    pub fn wr_dis_of_spipfra(&mut self) -> WrDisOfSpipfraW<Spipf07cSpec> {
        WrDisOfSpipfraW::new(self, 31)
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
