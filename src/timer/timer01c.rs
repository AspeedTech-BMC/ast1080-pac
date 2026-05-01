#[doc = "Register `TIMER01C` reader"]
pub type R = crate::R<Timer01cSpec>;
#[doc = "Register `TIMER01C` writer"]
pub type W = crate::W<Timer01cSpec>;
#[doc = "Field `WrProtOfTMC00` reader - Write Protection of TMC00"]
pub type WrProtOfTmc00R = crate::BitReader;
#[doc = "Field `WrProtOfTMC00` writer - Write Protection of TMC00"]
pub type WrProtOfTmc00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfTMC04` reader - Write Protection of TMC04"]
pub type WrProtOfTmc04R = crate::BitReader;
#[doc = "Field `WrProtOfTMC04` writer - Write Protection of TMC04"]
pub type WrProtOfTmc04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfTMC08` reader - Write Protection of TMC08"]
pub type WrProtOfTmc08R = crate::BitReader;
#[doc = "Field `WrProtOfTMC08` writer - Write Protection of TMC08"]
pub type WrProtOfTmc08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfTMC0C` reader - Write Protection of TMC0C"]
pub type WrProtOfTmc0cR = crate::BitReader;
#[doc = "Field `WrProtOfTMC0C` writer - Write Protection of TMC0C"]
pub type WrProtOfTmc0cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfTMC10` reader - Write Protection of TMC10"]
pub type WrProtOfTmc10R = crate::BitReader;
#[doc = "Field `WrProtOfTMC10` writer - Write Protection of TMC10"]
pub type WrProtOfTmc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfTMC14` reader - Write Protection of TMC14"]
pub type WrProtOfTmc14R = crate::BitReader;
#[doc = "Field `WrProtOfTMC14` writer - Write Protection of TMC14"]
pub type WrProtOfTmc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Write Protection of TMC00"]
    #[inline(always)]
    pub fn wr_prot_of_tmc00(&self) -> WrProtOfTmc00R {
        WrProtOfTmc00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection of TMC04"]
    #[inline(always)]
    pub fn wr_prot_of_tmc04(&self) -> WrProtOfTmc04R {
        WrProtOfTmc04R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protection of TMC08"]
    #[inline(always)]
    pub fn wr_prot_of_tmc08(&self) -> WrProtOfTmc08R {
        WrProtOfTmc08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Protection of TMC0C"]
    #[inline(always)]
    pub fn wr_prot_of_tmc0c(&self) -> WrProtOfTmc0cR {
        WrProtOfTmc0cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protection of TMC10"]
    #[inline(always)]
    pub fn wr_prot_of_tmc10(&self) -> WrProtOfTmc10R {
        WrProtOfTmc10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection of TMC14"]
    #[inline(always)]
    pub fn wr_prot_of_tmc14(&self) -> WrProtOfTmc14R {
        WrProtOfTmc14R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection of TMC00"]
    #[inline(always)]
    pub fn wr_prot_of_tmc00(&mut self) -> WrProtOfTmc00W<Timer01cSpec> {
        WrProtOfTmc00W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protection of TMC04"]
    #[inline(always)]
    pub fn wr_prot_of_tmc04(&mut self) -> WrProtOfTmc04W<Timer01cSpec> {
        WrProtOfTmc04W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protection of TMC08"]
    #[inline(always)]
    pub fn wr_prot_of_tmc08(&mut self) -> WrProtOfTmc08W<Timer01cSpec> {
        WrProtOfTmc08W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protection of TMC0C"]
    #[inline(always)]
    pub fn wr_prot_of_tmc0c(&mut self) -> WrProtOfTmc0cW<Timer01cSpec> {
        WrProtOfTmc0cW::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protection of TMC10"]
    #[inline(always)]
    pub fn wr_prot_of_tmc10(&mut self) -> WrProtOfTmc10W<Timer01cSpec> {
        WrProtOfTmc10W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection of TMC14"]
    #[inline(always)]
    pub fn wr_prot_of_tmc14(&mut self) -> WrProtOfTmc14W<Timer01cSpec> {
        WrProtOfTmc14W::new(self, 5)
    }
}
#[doc = "Conter Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer01cSpec;
impl crate::RegisterSpec for Timer01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer01c::R`](R) reader structure"]
impl crate::Readable for Timer01cSpec {}
#[doc = "`write(|w| ..)` method takes [`timer01c::W`](W) writer structure"]
impl crate::Writable for Timer01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER01C to value 0"]
impl crate::Resettable for Timer01cSpec {}
