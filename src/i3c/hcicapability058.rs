#[doc = "Register `HCICAPABILITY058` reader"]
pub type R = crate::R<Hcicapability058Spec>;
#[doc = "Register `HCICAPABILITY058` writer"]
pub type W = crate::W<Hcicapability058Spec>;
#[doc = "Field `REGNOTIFYHJREJECTED` reader - REG_NOTIFY_HJ_REJECTED"]
pub type RegnotifyhjrejectedR = crate::BitReader;
#[doc = "Field `REGNOTIFYHJREJECTED` writer - REG_NOTIFY_HJ_REJECTED"]
pub type RegnotifyhjrejectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGNOTIFYCRRREJECTED` reader - REG_NOTIFY_CRR_REJECTED"]
pub type RegnotifycrrrejectedR = crate::BitReader;
#[doc = "Field `REGNOTIFYCRRREJECTED` writer - REG_NOTIFY_CRR_REJECTED"]
pub type RegnotifycrrrejectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGNOTIFYIBIREJECTED` reader - REG_NOTIFY_IBI_REJECTED"]
pub type RegnotifyibirejectedR = crate::BitReader;
#[doc = "Field `REGNOTIFYIBIREJECTED` writer - REG_NOTIFY_IBI_REJECTED"]
pub type RegnotifyibirejectedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_NOTIFY_HJ_REJECTED"]
    #[inline(always)]
    pub fn regnotifyhjrejected(&self) -> RegnotifyhjrejectedR {
        RegnotifyhjrejectedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_NOTIFY_CRR_REJECTED"]
    #[inline(always)]
    pub fn regnotifycrrrejected(&self) -> RegnotifycrrrejectedR {
        RegnotifycrrrejectedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_NOTIFY_IBI_REJECTED"]
    #[inline(always)]
    pub fn regnotifyibirejected(&self) -> RegnotifyibirejectedR {
        RegnotifyibirejectedR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_NOTIFY_HJ_REJECTED"]
    #[inline(always)]
    pub fn regnotifyhjrejected(&mut self) -> RegnotifyhjrejectedW<Hcicapability058Spec> {
        RegnotifyhjrejectedW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_NOTIFY_CRR_REJECTED"]
    #[inline(always)]
    pub fn regnotifycrrrejected(&mut self) -> RegnotifycrrrejectedW<Hcicapability058Spec> {
        RegnotifycrrrejectedW::new(self, 1)
    }
    #[doc = "Bit 3 - REG_NOTIFY_IBI_REJECTED"]
    #[inline(always)]
    pub fn regnotifyibirejected(&mut self) -> RegnotifyibirejectedW<Hcicapability058Spec> {
        RegnotifyibirejectedW::new(self, 3)
    }
}
#[doc = "IBI\\_NOTIFY\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability058Spec;
impl crate::RegisterSpec for Hcicapability058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability058::R`](R) reader structure"]
impl crate::Readable for Hcicapability058Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability058::W`](W) writer structure"]
impl crate::Writable for Hcicapability058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY058 to value 0"]
impl crate::Resettable for Hcicapability058Spec {}
