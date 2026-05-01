#[doc = "Register `IPC00C` reader"]
pub type R = crate::R<Ipc00cSpec>;
#[doc = "Register `IPC00C` writer"]
pub type W = crate::W<Ipc00cSpec>;
impl W {}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc00cSpec;
impl crate::RegisterSpec for Ipc00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc00c::R`](R) reader structure"]
impl crate::Readable for Ipc00cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc00c::W`](W) writer structure"]
impl crate::Writable for Ipc00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC00C to value 0"]
impl crate::Resettable for Ipc00cSpec {}
