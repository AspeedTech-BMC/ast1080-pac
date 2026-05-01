#[doc = "Register `SCU310` reader"]
pub type R = crate::R<Scu310Spec>;
#[doc = "Register `SCU310` writer"]
pub type W = crate::W<Scu310Spec>;
#[doc = "Field `SCUDIPLLEN` reader - SCU_DIPLL_EN"]
pub type ScudipllenR = crate::BitReader;
#[doc = "Field `SCUDIPLLEN` writer - SCU_DIPLL_EN"]
pub type ScudipllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLDCOEN` reader - SCU_DIPLL_DCO_EN"]
pub type ScudiplldcoenR = crate::BitReader;
#[doc = "Field `SCUDIPLLDCOEN` writer - SCU_DIPLL_DCO_EN"]
pub type ScudiplldcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLFNMODE` reader - SCU_DIPLL_FN_MODE"]
pub type ScudipllfnmodeR = crate::BitReader;
#[doc = "Field `SCUDIPLLFNMODE` writer - SCU_DIPLL_FN_MODE"]
pub type ScudipllfnmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLSSCEN` reader - SCU_DIPLL_SSC_EN"]
pub type ScudipllsscenR = crate::BitReader;
#[doc = "Field `SCUDIPLLSSCEN` writer - SCU_DIPLL_SSC_EN"]
pub type ScudipllsscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLCHGEN` reader - SCU_DIPLL_CHG_EN"]
pub type ScudipllchgenR = crate::BitReader;
#[doc = "Field `SCUDIPLLCHGEN` writer - SCU_DIPLL_CHG_EN"]
pub type ScudipllchgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLDIVPRE` reader - SCU_DIPLL_DIV_PRE"]
pub type ScudiplldivpreR = crate::FieldReader;
#[doc = "Field `SCUDIPLLDIVPRE` writer - SCU_DIPLL_DIV_PRE"]
pub type ScudiplldivpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUDIPLLDIVN` reader - SCU_DIPLL_DIV_N"]
pub type ScudiplldivnR = crate::FieldReader<u16>;
#[doc = "Field `SCUDIPLLDIVN` writer - SCU_DIPLL_DIV_N"]
pub type ScudiplldivnW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SCUDIPLLDCOCONEN` reader - SCU_DIPLL_DCO_CONEN"]
pub type ScudiplldcoconenR = crate::BitReader;
#[doc = "Field `SCUDIPLLDCOCONEN` writer - SCU_DIPLL_DCO_CONEN"]
pub type ScudiplldcoconenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLLPF` reader - SCU_DIPLL_LPF"]
pub type ScudiplllpfR = crate::FieldReader<u16>;
#[doc = "Field `SCUDIPLLLPF` writer - SCU_DIPLL_LPF"]
pub type ScudiplllpfW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - SCU_DIPLL_EN"]
    #[inline(always)]
    pub fn scudipllen(&self) -> ScudipllenR {
        ScudipllenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_DIPLL_DCO_EN"]
    #[inline(always)]
    pub fn scudiplldcoen(&self) -> ScudiplldcoenR {
        ScudiplldcoenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIPLL_FN_MODE"]
    #[inline(always)]
    pub fn scudipllfnmode(&self) -> ScudipllfnmodeR {
        ScudipllfnmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_DIPLL_SSC_EN"]
    #[inline(always)]
    pub fn scudipllsscen(&self) -> ScudipllsscenR {
        ScudipllsscenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_DIPLL_CHG_EN"]
    #[inline(always)]
    pub fn scudipllchgen(&self) -> ScudipllchgenR {
        ScudipllchgenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - SCU_DIPLL_DIV_PRE"]
    #[inline(always)]
    pub fn scudiplldivpre(&self) -> ScudiplldivpreR {
        ScudiplldivpreR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:17 - SCU_DIPLL_DIV_N"]
    #[inline(always)]
    pub fn scudiplldivn(&self) -> ScudiplldivnR {
        ScudiplldivnR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - SCU_DIPLL_DCO_CONEN"]
    #[inline(always)]
    pub fn scudiplldcoconen(&self) -> ScudiplldcoconenR {
        ScudiplldcoconenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:27 - SCU_DIPLL_LPF"]
    #[inline(always)]
    pub fn scudiplllpf(&self) -> ScudiplllpfR {
        ScudiplllpfR::new(((self.bits >> 19) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIPLL_EN"]
    #[inline(always)]
    pub fn scudipllen(&mut self) -> ScudipllenW<Scu310Spec> {
        ScudipllenW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_DIPLL_DCO_EN"]
    #[inline(always)]
    pub fn scudiplldcoen(&mut self) -> ScudiplldcoenW<Scu310Spec> {
        ScudiplldcoenW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_DIPLL_FN_MODE"]
    #[inline(always)]
    pub fn scudipllfnmode(&mut self) -> ScudipllfnmodeW<Scu310Spec> {
        ScudipllfnmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_DIPLL_SSC_EN"]
    #[inline(always)]
    pub fn scudipllsscen(&mut self) -> ScudipllsscenW<Scu310Spec> {
        ScudipllsscenW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_DIPLL_CHG_EN"]
    #[inline(always)]
    pub fn scudipllchgen(&mut self) -> ScudipllchgenW<Scu310Spec> {
        ScudipllchgenW::new(self, 4)
    }
    #[doc = "Bits 5:8 - SCU_DIPLL_DIV_PRE"]
    #[inline(always)]
    pub fn scudiplldivpre(&mut self) -> ScudiplldivpreW<Scu310Spec> {
        ScudiplldivpreW::new(self, 5)
    }
    #[doc = "Bits 9:17 - SCU_DIPLL_DIV_N"]
    #[inline(always)]
    pub fn scudiplldivn(&mut self) -> ScudiplldivnW<Scu310Spec> {
        ScudiplldivnW::new(self, 9)
    }
    #[doc = "Bit 18 - SCU_DIPLL_DCO_CONEN"]
    #[inline(always)]
    pub fn scudiplldcoconen(&mut self) -> ScudiplldcoconenW<Scu310Spec> {
        ScudiplldcoconenW::new(self, 18)
    }
    #[doc = "Bits 19:27 - SCU_DIPLL_LPF"]
    #[inline(always)]
    pub fn scudiplllpf(&mut self) -> ScudiplllpfW<Scu310Spec> {
        ScudiplllpfW::new(self, 19)
    }
}
#[doc = "DIPLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu310Spec;
impl crate::RegisterSpec for Scu310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu310::R`](R) reader structure"]
impl crate::Readable for Scu310Spec {}
#[doc = "`write(|w| ..)` method takes [`scu310::W`](W) writer structure"]
impl crate::Writable for Scu310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU310 to value 0x6020"]
impl crate::Resettable for Scu310Spec {
    const RESET_VALUE: u32 = 0x6020;
}
