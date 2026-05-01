#[doc = "Register `I3CCONTROL024` reader"]
pub type R = crate::R<I3ccontrol024Spec>;
#[doc = "Register `I3CCONTROL024` writer"]
pub type W = crate::W<I3ccontrol024Spec>;
#[doc = "Field `REGAUTOCMDMODE1` reader - REG_AUTOCMD_MODE_1"]
pub type Regautocmdmode1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE1` writer - REG_AUTOCMD_MODE_1"]
pub type Regautocmdmode1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE1` reader - REG_AUTOCMD_VALUE_1"]
pub type Regautocmdvalue1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE1` writer - REG_AUTOCMD_VALUE_1"]
pub type Regautocmdvalue1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK1` reader - REG_AUTOCMD_MASK_1"]
pub type Regautocmdmask1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK1` writer - REG_AUTOCMD_MASK_1"]
pub type Regautocmdmask1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_1"]
    #[inline(always)]
    pub fn regautocmdmode1(&self) -> Regautocmdmode1R {
        Regautocmdmode1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_1"]
    #[inline(always)]
    pub fn regautocmdvalue1(&self) -> Regautocmdvalue1R {
        Regautocmdvalue1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_1"]
    #[inline(always)]
    pub fn regautocmdmask1(&self) -> Regautocmdmask1R {
        Regautocmdmask1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_1"]
    #[inline(always)]
    pub fn regautocmdmode1(&mut self) -> Regautocmdmode1W<I3ccontrol024Spec> {
        Regautocmdmode1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_1"]
    #[inline(always)]
    pub fn regautocmdvalue1(&mut self) -> Regautocmdvalue1W<I3ccontrol024Spec> {
        Regautocmdvalue1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_1"]
    #[inline(always)]
    pub fn regautocmdmask1(&mut self) -> Regautocmdmask1W<I3ccontrol024Spec> {
        Regautocmdmask1W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol024Spec;
impl crate::RegisterSpec for I3ccontrol024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol024::R`](R) reader structure"]
impl crate::Readable for I3ccontrol024Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol024::W`](W) writer structure"]
impl crate::Writable for I3ccontrol024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL024 to value 0xff00"]
impl crate::Resettable for I3ccontrol024Spec {
    const RESET_VALUE: u32 = 0xff00;
}
