#[doc = "Register `UHCI080` reader"]
pub type R = crate::R<Uhci080Spec>;
#[doc = "Register `UHCI080` writer"]
pub type W = crate::W<Uhci080Spec>;
#[doc = "Field `FrameListCurIndexFrameNumber` reader - Frame List Current Index/Frame Number"]
pub type FrameListCurIndexFrameNumberR = crate::FieldReader<u16>;
#[doc = "Field `FrameListCurIndexFrameNumber` writer - Frame List Current Index/Frame Number"]
pub type FrameListCurIndexFrameNumberW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:10 - Frame List Current Index/Frame Number"]
    #[inline(always)]
    pub fn frame_list_cur_index_frame_number(&self) -> FrameListCurIndexFrameNumberR {
        FrameListCurIndexFrameNumberR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame List Current Index/Frame Number"]
    #[inline(always)]
    pub fn frame_list_cur_index_frame_number(
        &mut self,
    ) -> FrameListCurIndexFrameNumberW<Uhci080Spec> {
        FrameListCurIndexFrameNumberW::new(self, 0)
    }
}
#[doc = "Frame Number Register (FRNUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci080Spec;
impl crate::RegisterSpec for Uhci080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci080::R`](R) reader structure"]
impl crate::Readable for Uhci080Spec {}
#[doc = "`write(|w| ..)` method takes [`uhci080::W`](W) writer structure"]
impl crate::Writable for Uhci080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI080 to value 0"]
impl crate::Resettable for Uhci080Spec {}
