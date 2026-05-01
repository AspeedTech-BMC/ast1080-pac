#[doc = "Register `SCU320` reader"]
pub type R = crate::R<Scu320Spec>;
#[doc = "Register `SCU320` writer"]
pub type W = crate::W<Scu320Spec>;
#[doc = "Field `SCUDIPLLBWCON` reader - SCU_DIPLL_BW_CON"]
pub type ScudipllbwconR = crate::FieldReader;
#[doc = "Field `SCUDIPLLBWCON` writer - SCU_DIPLL_BW_CON"]
pub type ScudipllbwconW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUDIPLLDIVSYS` reader - SCU_DIPLL_DIV_SYS"]
pub type ScudiplldivsysR = crate::BitReader;
#[doc = "Field `SCUDIPLLDIVSYS` writer - SCU_DIPLL_DIV_SYS"]
pub type ScudiplldivsysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLDIVNSEL` reader - SCU_DIPLL_DIV_N_SEL"]
pub type ScudiplldivnselR = crate::BitReader;
#[doc = "Field `SCUDIPLLDIVNSEL` writer - SCU_DIPLL_DIV_N_SEL"]
pub type ScudiplldivnselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLKI` reader - SCU_DIPLL_KI"]
pub type ScudipllkiR = crate::FieldReader;
#[doc = "Field `SCUDIPLLKI` writer - SCU_DIPLL_KI"]
pub type ScudipllkiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCUDIPLLRTDLY` reader - SCU_DIPLL_RT_DLY"]
pub type ScudipllrtdlyR = crate::BitReader;
#[doc = "Field `SCUDIPLLRTDLY` writer - SCU_DIPLL_RT_DLY"]
pub type ScudipllrtdlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLFMEN` reader - SCU_DIPLL_FM_EN"]
pub type ScudipllfmenR = crate::BitReader;
#[doc = "Field `SCUDIPLLFMEN` writer - SCU_DIPLL_FM_EN"]
pub type ScudipllfmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLFMTOR` reader - SCU_DIPLL_FM_TOR"]
pub type ScudipllfmtorR = crate::FieldReader;
#[doc = "Field `SCUDIPLLFMTOR` writer - SCU_DIPLL_FM_TOR"]
pub type ScudipllfmtorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCUDIPLLFDDIV` reader - SCU_DIPLL_FD_DIV"]
pub type ScudipllfddivR = crate::FieldReader;
#[doc = "Field `SCUDIPLLFDDIV` writer - SCU_DIPLL_FD_DIV"]
pub type ScudipllfddivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCUDIPLLFHEN` reader - SCU_DIPLL_FH_EN"]
pub type ScudipllfhenR = crate::BitReader;
#[doc = "Field `SCUDIPLLFHEN` writer - SCU_DIPLL_FH_EN"]
pub type ScudipllfhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLDEN` reader - SCU_DIPLL_DEN"]
pub type ScudiplldenR = crate::BitReader;
#[doc = "Field `SCUDIPLLDEN` writer - SCU_DIPLL_DEN"]
pub type ScudiplldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLFASTLOCKEN` reader - SCU_DIPLL_FAST_LOCK_EN"]
pub type ScudipllfastlockenR = crate::BitReader;
#[doc = "Field `SCUDIPLLFASTLOCKEN` writer - SCU_DIPLL_FAST_LOCK_EN"]
pub type ScudipllfastlockenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - SCU_DIPLL_BW_CON"]
    #[inline(always)]
    pub fn scudipllbwcon(&self) -> ScudipllbwconR {
        ScudipllbwconR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - SCU_DIPLL_DIV_SYS"]
    #[inline(always)]
    pub fn scudiplldivsys(&self) -> ScudiplldivsysR {
        ScudiplldivsysR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_DIPLL_DIV_N_SEL"]
    #[inline(always)]
    pub fn scudiplldivnsel(&self) -> ScudiplldivnselR {
        ScudiplldivnselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SCU_DIPLL_KI"]
    #[inline(always)]
    pub fn scudipllki(&self) -> ScudipllkiR {
        ScudipllkiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SCU_DIPLL_RT_DLY"]
    #[inline(always)]
    pub fn scudipllrtdly(&self) -> ScudipllrtdlyR {
        ScudipllrtdlyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_DIPLL_FM_EN"]
    #[inline(always)]
    pub fn scudipllfmen(&self) -> ScudipllfmenR {
        ScudipllfmenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - SCU_DIPLL_FM_TOR"]
    #[inline(always)]
    pub fn scudipllfmtor(&self) -> ScudipllfmtorR {
        ScudipllfmtorR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - SCU_DIPLL_FD_DIV"]
    #[inline(always)]
    pub fn scudipllfddiv(&self) -> ScudipllfddivR {
        ScudipllfddivR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - SCU_DIPLL_FH_EN"]
    #[inline(always)]
    pub fn scudipllfhen(&self) -> ScudipllfhenR {
        ScudipllfhenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_DIPLL_DEN"]
    #[inline(always)]
    pub fn scudipllden(&self) -> ScudiplldenR {
        ScudiplldenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_DIPLL_FAST_LOCK_EN"]
    #[inline(always)]
    pub fn scudipllfastlocken(&self) -> ScudipllfastlockenR {
        ScudipllfastlockenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SCU_DIPLL_BW_CON"]
    #[inline(always)]
    pub fn scudipllbwcon(&mut self) -> ScudipllbwconW<Scu320Spec> {
        ScudipllbwconW::new(self, 0)
    }
    #[doc = "Bit 4 - SCU_DIPLL_DIV_SYS"]
    #[inline(always)]
    pub fn scudiplldivsys(&mut self) -> ScudiplldivsysW<Scu320Spec> {
        ScudiplldivsysW::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_DIPLL_DIV_N_SEL"]
    #[inline(always)]
    pub fn scudiplldivnsel(&mut self) -> ScudiplldivnselW<Scu320Spec> {
        ScudiplldivnselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - SCU_DIPLL_KI"]
    #[inline(always)]
    pub fn scudipllki(&mut self) -> ScudipllkiW<Scu320Spec> {
        ScudipllkiW::new(self, 6)
    }
    #[doc = "Bit 8 - SCU_DIPLL_RT_DLY"]
    #[inline(always)]
    pub fn scudipllrtdly(&mut self) -> ScudipllrtdlyW<Scu320Spec> {
        ScudipllrtdlyW::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_DIPLL_FM_EN"]
    #[inline(always)]
    pub fn scudipllfmen(&mut self) -> ScudipllfmenW<Scu320Spec> {
        ScudipllfmenW::new(self, 9)
    }
    #[doc = "Bits 10:11 - SCU_DIPLL_FM_TOR"]
    #[inline(always)]
    pub fn scudipllfmtor(&mut self) -> ScudipllfmtorW<Scu320Spec> {
        ScudipllfmtorW::new(self, 10)
    }
    #[doc = "Bits 12:14 - SCU_DIPLL_FD_DIV"]
    #[inline(always)]
    pub fn scudipllfddiv(&mut self) -> ScudipllfddivW<Scu320Spec> {
        ScudipllfddivW::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_DIPLL_FH_EN"]
    #[inline(always)]
    pub fn scudipllfhen(&mut self) -> ScudipllfhenW<Scu320Spec> {
        ScudipllfhenW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_DIPLL_DEN"]
    #[inline(always)]
    pub fn scudipllden(&mut self) -> ScudiplldenW<Scu320Spec> {
        ScudiplldenW::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_DIPLL_FAST_LOCK_EN"]
    #[inline(always)]
    pub fn scudipllfastlocken(&mut self) -> ScudipllfastlockenW<Scu320Spec> {
        ScudipllfastlockenW::new(self, 17)
    }
}
#[doc = "DIPLL Parameter Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu320::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu320::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu320Spec;
impl crate::RegisterSpec for Scu320Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu320::R`](R) reader structure"]
impl crate::Readable for Scu320Spec {}
#[doc = "`write(|w| ..)` method takes [`scu320::W`](W) writer structure"]
impl crate::Writable for Scu320Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU320 to value 0x0156"]
impl crate::Resettable for Scu320Spec {
    const RESET_VALUE: u32 = 0x0156;
}
