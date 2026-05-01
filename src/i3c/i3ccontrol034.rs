#[doc = "Register `I3CCONTROL034` reader"]
pub type R = crate::R<I3ccontrol034Spec>;
#[doc = "Register `I3CCONTROL034` writer"]
pub type W = crate::W<I3ccontrol034Spec>;
#[doc = "Field `REGAUTOCMDMODE5` reader - REG_AUTOCMD_MODE_5"]
pub type Regautocmdmode5R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE5` writer - REG_AUTOCMD_MODE_5"]
pub type Regautocmdmode5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE5` reader - REG_AUTOCMD_VALUE_5"]
pub type Regautocmdvalue5R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE5` writer - REG_AUTOCMD_VALUE_5"]
pub type Regautocmdvalue5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK5` reader - REG_AUTOCMD_MASK_5"]
pub type Regautocmdmask5R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK5` writer - REG_AUTOCMD_MASK_5"]
pub type Regautocmdmask5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_5"]
    #[inline(always)]
    pub fn regautocmdmode5(&self) -> Regautocmdmode5R {
        Regautocmdmode5R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_5"]
    #[inline(always)]
    pub fn regautocmdvalue5(&self) -> Regautocmdvalue5R {
        Regautocmdvalue5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_5"]
    #[inline(always)]
    pub fn regautocmdmask5(&self) -> Regautocmdmask5R {
        Regautocmdmask5R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_5"]
    #[inline(always)]
    pub fn regautocmdmode5(&mut self) -> Regautocmdmode5W<I3ccontrol034Spec> {
        Regautocmdmode5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_5"]
    #[inline(always)]
    pub fn regautocmdvalue5(&mut self) -> Regautocmdvalue5W<I3ccontrol034Spec> {
        Regautocmdvalue5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_5"]
    #[inline(always)]
    pub fn regautocmdmask5(&mut self) -> Regautocmdmask5W<I3ccontrol034Spec> {
        Regautocmdmask5W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_5\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol034Spec;
impl crate::RegisterSpec for I3ccontrol034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol034::R`](R) reader structure"]
impl crate::Readable for I3ccontrol034Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol034::W`](W) writer structure"]
impl crate::Writable for I3ccontrol034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL034 to value 0xff00"]
impl crate::Resettable for I3ccontrol034Spec {
    const RESET_VALUE: u32 = 0xff00;
}
