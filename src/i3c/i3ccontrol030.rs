#[doc = "Register `I3CCONTROL030` reader"]
pub type R = crate::R<I3ccontrol030Spec>;
#[doc = "Register `I3CCONTROL030` writer"]
pub type W = crate::W<I3ccontrol030Spec>;
#[doc = "Field `REGAUTOCMDMODE4` reader - REG_AUTOCMD_MODE_4"]
pub type Regautocmdmode4R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE4` writer - REG_AUTOCMD_MODE_4"]
pub type Regautocmdmode4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE4` reader - REG_AUTOCMD_VALUE_4"]
pub type Regautocmdvalue4R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE4` writer - REG_AUTOCMD_VALUE_4"]
pub type Regautocmdvalue4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK4` reader - REG_AUTOCMD_MASK_4"]
pub type Regautocmdmask4R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK4` writer - REG_AUTOCMD_MASK_4"]
pub type Regautocmdmask4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_4"]
    #[inline(always)]
    pub fn regautocmdmode4(&self) -> Regautocmdmode4R {
        Regautocmdmode4R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_4"]
    #[inline(always)]
    pub fn regautocmdvalue4(&self) -> Regautocmdvalue4R {
        Regautocmdvalue4R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_4"]
    #[inline(always)]
    pub fn regautocmdmask4(&self) -> Regautocmdmask4R {
        Regautocmdmask4R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_4"]
    #[inline(always)]
    pub fn regautocmdmode4(&mut self) -> Regautocmdmode4W<I3ccontrol030Spec> {
        Regautocmdmode4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_4"]
    #[inline(always)]
    pub fn regautocmdvalue4(&mut self) -> Regautocmdvalue4W<I3ccontrol030Spec> {
        Regautocmdvalue4W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_4"]
    #[inline(always)]
    pub fn regautocmdmask4(&mut self) -> Regautocmdmask4W<I3ccontrol030Spec> {
        Regautocmdmask4W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol030Spec;
impl crate::RegisterSpec for I3ccontrol030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol030::R`](R) reader structure"]
impl crate::Readable for I3ccontrol030Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol030::W`](W) writer structure"]
impl crate::Writable for I3ccontrol030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL030 to value 0xff00"]
impl crate::Resettable for I3ccontrol030Spec {
    const RESET_VALUE: u32 = 0xff00;
}
