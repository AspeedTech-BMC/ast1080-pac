#[doc = "Register `I3CCONTROL03C` reader"]
pub type R = crate::R<I3ccontrol03cSpec>;
#[doc = "Register `I3CCONTROL03C` writer"]
pub type W = crate::W<I3ccontrol03cSpec>;
#[doc = "Field `REGAUTOCMDMODE7` reader - REG_AUTOCMD_MODE_7"]
pub type Regautocmdmode7R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE7` writer - REG_AUTOCMD_MODE_7"]
pub type Regautocmdmode7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE7` reader - REG_AUTOCMD_VALUE_7"]
pub type Regautocmdvalue7R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE7` writer - REG_AUTOCMD_VALUE_7"]
pub type Regautocmdvalue7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK7` reader - REG_AUTOCMD_MASK_7"]
pub type Regautocmdmask7R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK7` writer - REG_AUTOCMD_MASK_7"]
pub type Regautocmdmask7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_7"]
    #[inline(always)]
    pub fn regautocmdmode7(&self) -> Regautocmdmode7R {
        Regautocmdmode7R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_7"]
    #[inline(always)]
    pub fn regautocmdvalue7(&self) -> Regautocmdvalue7R {
        Regautocmdvalue7R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_7"]
    #[inline(always)]
    pub fn regautocmdmask7(&self) -> Regautocmdmask7R {
        Regautocmdmask7R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_7"]
    #[inline(always)]
    pub fn regautocmdmode7(&mut self) -> Regautocmdmode7W<I3ccontrol03cSpec> {
        Regautocmdmode7W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_7"]
    #[inline(always)]
    pub fn regautocmdvalue7(&mut self) -> Regautocmdvalue7W<I3ccontrol03cSpec> {
        Regautocmdvalue7W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_7"]
    #[inline(always)]
    pub fn regautocmdmask7(&mut self) -> Regautocmdmask7W<I3ccontrol03cSpec> {
        Regautocmdmask7W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_7\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol03cSpec;
impl crate::RegisterSpec for I3ccontrol03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol03c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol03cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol03c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL03C to value 0xff00"]
impl crate::Resettable for I3ccontrol03cSpec {
    const RESET_VALUE: u32 = 0xff00;
}
